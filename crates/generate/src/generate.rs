use crate::{
    parse::{BinaryOpKind, Expr, Stmt},
    Intrinsic,
};
use eyre::{bail, Context, OptionExt, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use syn::Block;

// If a function is polymorphic, the argument type will be "mangled" into the name.
const POLYMORPHIC_FNS: &[&str] = &["ABS"];

pub fn generate(intrinsics: &[Intrinsic]) -> Result<syn::File> {
    let blanket: syn::ItemImpl = syn::parse_quote! {
        impl<C: super::Core> Intrinsics for C {}
    };

    let mut trait_def: syn::ItemTrait = syn::parse_quote! {
        pub trait Intrinsics: super::Core {}
    };
    for intr in intrinsics {
        let item = generate_intr(intr).wrap_err_with(|| format!("generating `{}`", intr.name))?;
        trait_def.items.push(item);
    }

    let soft_arch =
        generate_soft_arch_module(intrinsics).wrap_err("generating soft_arch module")?;

    let test = generate_test_module(intrinsics).wrap_err("generating test module")?;

    let mut file: syn::File = syn::parse_quote! { #![allow(unused_parens)] };
    file.items = vec![
        blanket.into(),
        trait_def.into(),
        soft_arch.into(),
        test.into(),
    ];

    Ok(file)
}

fn generate_soft_arch_module(intrinsics: &[Intrinsic]) -> Result<syn::ItemMod> {
    let mut module: syn::ItemMod = syn::parse_quote! {
        pub mod soft_arch {
            pub use super::super::soft_arch_types::*;
            use super::Intrinsics;
        }
    };

    for intr in intrinsics {
        let item = generate_intr_soft_arch_wrap(intr)
            .wrap_err_with(|| format!("generating `{}`", intr.name))?;
        module.content.as_mut().unwrap().1.push(item.into());
    }

    Ok(module)
}

fn generate_test_module(intrinsics: &[Intrinsic]) -> Result<syn::ItemMod> {
    let mut module: syn::ItemMod = syn::parse_quote! {
        #[cfg(all(test, target_arch = "x86_64"))]
        pub mod tests {
            use super::super::compare_test_helper::hard_soft_same_128;
        }
    };

    let rng = &mut SmallRng::seed_from_u64(44);
    for intr in intrinsics {
        let item = generate_intr_test(intr, rng)
            .wrap_err_with(|| format!("generating soft_arch `{}`", intr.name))?;

        module.content.as_mut().unwrap().1.push(item.into());
    }

    Ok(module)
}

fn generate_intr(intr: &Intrinsic) -> Result<syn::TraitItem, eyre::Error> {
    eprintln!("generating {}...", intr.name);

    let body = generate_body(intr).wrap_err("generating body")?;

    signature(intr, body)
}

fn generate_intr_soft_arch_wrap(intr: &Intrinsic) -> Result<syn::ItemFn, eyre::Error> {
    eprintln!("generating soft_arch wrapper {}...", intr.name);

    let body = generate_body_soft_arch(intr).wrap_err("generating body")?;

    signature_soft_arch(intr, body)
}

fn generate_intr_test(intr: &Intrinsic, rng: &mut SmallRng) -> Result<syn::ItemFn, eyre::Error> {
    eprintln!("generating test {}...", intr.name);

    let name = ident(&intr.name);
    let body = generate_body_test(intr, rng).wrap_err("generating body")?;

    Ok(syn::parse_quote! {
        #[test]
        fn #name() {
            hard_soft_same_128! {
                #body
            }
        }
    })
}

#[derive(Default)]
struct BlockBuilder {
    tmp_count: u64,
    stmts: Vec<syn::Stmt>,
}
impl BlockBuilder {
    fn tmp(&mut self) -> syn::Ident {
        let r = format!("__tmp{}", self.tmp_count);
        self.tmp_count += 1;
        ident(&r)
    }
}

fn generate_body_soft_arch(intr: &Intrinsic) -> Result<syn::Block> {
    let mut block = BlockBuilder::default();

    block
        .stmts
        .push(syn::parse_quote! { let mut output = unsafe { std::mem::zeroed() }; });

    let name = ident(&intr.name);

    let args = intr.parameter.iter().map(|param| -> syn::Expr {
        let name = ident_opt_s(&param.varname).unwrap();
        syn::parse_quote! { #name as _ }
    });

    block.stmts.push(syn::parse_quote! {
        super::super::ValueCore.#name(&mut output, #(#args),*);
    });

    block
        .stmts
        .push(syn::Stmt::Expr(syn::parse_quote! { output }, None));
    let block = block.stmts;
    Ok(syn::parse_quote! {
        { #(#block)* }
    })
}

fn generate_body_test(intr: &Intrinsic, rng: &mut SmallRng) -> Result<syn::Block> {
    let mut block = BlockBuilder::default();

    let args = intr
        .parameter
        .iter()
        .map(|param| -> Result<syn::Expr> {
            let name = ident_opt_s(&param.varname).unwrap();
            let ty = map_type_to_rust(param.r#type.as_deref().unwrap());

            let random = random_value(ty, rng)?;
            block.stmts.push(syn::parse_quote! { let #name = #random; });
            Ok(syn::parse_quote! { #name })
        })
        .collect::<Result<Vec<syn::Expr>>>()
        .wrap_err("preparing arguments")?;

    let name = ident(&intr.name);
    block.stmts.push(syn::Stmt::Expr(
        syn::parse_quote!(#name( #(#args),* )),
        None,
    ));

    let block = block.stmts;
    Ok(syn::parse_quote! {
        { #(#block)* }
    })
}

fn random_value(ty: &str, rng: &mut SmallRng) -> Result<syn::Expr> {
    fn quote(n: impl quote::ToTokens) -> syn::Expr {
        syn::parse_quote! { #n }
    }
    Ok(match ty {
        "i8" => quote(rng.gen::<i8>()),
        "i16" => quote(rng.gen::<i16>()),
        "i32" => quote(rng.gen::<i32>()),
        "i64" => quote(rng.gen::<i64>()),
        "__m128i" => {
            let args = [
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
                quote(rng.gen::<i16>()),
            ];

            syn::parse_quote! {
                _mm_setr_epi16(#(#args),*)
            }
        }
        _ => bail!("unknown type for random value: {ty}"),
    })
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Type {
    Vector(VectorType),
    Scalar { elemty: ElementType },
}

/// A SIMD vector type like `16xi8 (__m128i)`
#[derive(Clone, Copy, PartialEq, Debug)]
struct VectorType {
    /// The amount of lanes, `16` in `16xi8 (__m128i)`.
    lanes: u64,
    /// The type of a single lane, `i8` in `16xi8 (__m128i)`.
    elem: ElementType,
    /// The raw Rust/C type, `__m128i` in `16xi8 (__m128i)`.
    raw_type: &'static str,
}

/// A single element in a vector.
/// For example in `16xi8 (__m128i)`, it would be `i8` (we do not care about signedness).
#[derive(Clone, Copy, PartialEq, Debug)]
struct ElementType {
    width: u64,
}

impl Type {
    fn of(etype: &str, ty: &str) -> Result<Self> {
        let etype_width = match etype {
            "SI8" => 8,
            "SI16" => 16,
            "SI32" => 32,
            "UI8" => 8,
            "UI16" => 16,
            "UI32" => 32,
            "UI64" => 64,
            _ => bail!("unknown element type: {etype}"),
        };
        let elem = ElementType { width: etype_width };

        let scalar = Type::Scalar { elemty: elem };

        Ok(match ty {
            "__m128i" => Type::Vector(VectorType {
                lanes: 128 / etype_width,
                elem,
                raw_type: "__m128i",
            }),
            "char" => scalar,
            "short" => scalar,
            "int" => scalar,
            "__int64" => scalar,
            _ => bail!("unknown type: {ty}"),
        })
    }

    fn rust_type(&self) -> String {
        match self {
            Type::Vector(v) => v.raw_type.to_owned(),
            Type::Scalar { elemty, .. } => elemty.rust_type(),
        }
    }

    fn expect_vector(&self) -> VectorType {
        let Self::Vector(ty) = *self else {
            panic!("expected vector, found scalar");
        };
        ty
    }
}

impl ElementType {
    fn rust_type(&self) -> String {
        format!("u{}", self.width)
    }
}

fn generate_body(instr: &Intrinsic) -> Result<syn::Block> {
    let opstmts = parse_op(instr)?;

    let type_of_ident = |ident: &str| -> Result<Type> {
        for param in &instr.parameter {
            if param.varname.as_deref() == Some(ident) {
                return Type::of(
                    param.etype.as_deref().ok_or_eyre("no param etype")?,
                    param.r#type.as_deref().ok_or_eyre("no param type")?,
                );
            }
        }

        if instr.ret.varname.as_deref() == Some(ident) {
            return Type::of(
                instr.ret.etype.as_deref().ok_or_eyre("no param etype")?,
                instr.ret.r#type.as_deref().ok_or_eyre("no param type")?,
            );
        }

        bail!("variable {ident} not found in pseudocode");
    };

    gen_block(opstmts, &type_of_ident)
}

fn gen_idx(
    method_prefix: &str,
    lhs: Expr,
    idx: Expr,
    type_of_ident: &impl Fn(&str) -> Result<Type>,
) -> Result<(syn::Ident, syn::Ident, syn::Expr, VectorType)> {
    let Expr::Ident(identifier) = lhs else {
        bail!("lhs of indexing must be identifier");
    };
    let Expr::Range { left, right } = idx else {
        bail!("idx argument must be range");
    };

    let ty = type_of_ident(&identifier)?.expect_vector();

    let (lane_idx, size): (syn::Expr, _) = match (*left, *right) {
        (Expr::Int(high), Expr::Int(low)) => {
            if high < low {
                bail!("range must be HIGH:LOW, but was {high}:{low}");
            }
            let size = high - low + 1; // (inclusive)

            let lane_idx = low / ty.elem.width;

            (syn::parse_quote! { #lane_idx }, size)
        }
        (
            Expr::BinaryOp {
                op: BinaryOpKind::Add,
                lhs,
                rhs,
            },
            Expr::Ident(low),
        ) => {
            let Expr::Ident(ref high_ident) = *lhs else {
                bail!("lhs of lhs of + indexing must be ident, was {rhs:?}");
            };
            let Expr::Int(ref high_offset) = *rhs else {
                bail!("rhs of lhs of + indexing must be ident, was {lhs:?}");
            };

            if *high_ident != low {
                bail!("{high_ident} != {low}");
            }
            let size = high_offset + 1;
            let identifier = ident(&low);
            (syn::parse_quote! { ( #identifier / #size ) }, size)
        }
        _ => bail!("unknown range indexing arguments"),
    };

    if !size.is_power_of_two() {
        bail!("indexing size must be power of two");
    }
    if size != ty.elem.width {
        bail!(
            "unsupported not-direct element indexing, size={size}, element size={}",
            ty.elem.width
        );
    }

    let identifier = ident(&identifier);
    let method = ident(&format!(
        "{method_prefix}_lane_{}_{}",
        ty.raw_type,
        ty.elem.rust_type()
    ));
    Ok((identifier, method, lane_idx, ty))
}

fn gen_block(opstmts: Vec<Stmt>, type_of_ident: &impl Fn(&str) -> Result<Type>) -> Result<Block> {
    let mut block = BlockBuilder::default();

    for stmt in opstmts {
        match stmt {
            Stmt::Assign {
                lhs: Expr::Index { lhs, idx },
                rhs,
            } => {
                let (identifier, method, lane_idx, _) = gen_idx("set", *lhs, *idx, type_of_ident)?;
                let expr = gen_expr_tmp(&mut block, rhs, &type_of_ident)?.0;

                block.stmts.push(syn::parse_quote! {
                    self.#method(#identifier, #lane_idx, #expr);
                });
            }
            Stmt::Assign {
                lhs: Expr::Ident(lhs),
                rhs,
            } => {
                let rhs = gen_expr_tmp(&mut block, rhs, type_of_ident)?.0;

                let exists = type_of_ident(&lhs).is_ok();

                let lhs = ident(&lhs);
                let stmt = if exists {
                    syn::parse_quote! { #lhs = #rhs; }
                } else {
                    syn::parse_quote! { let #lhs = #rhs; }
                };

                block.stmts.push(stmt);
            }
            Stmt::For {
                counter,
                from,
                to,
                body,
            } => {
                let counter = ident(&counter);
                let mut for_: syn::ExprForLoop =
                    syn::parse_quote! { for #counter in #from..=#to {} };

                let body = gen_block(body, type_of_ident)?;
                for_.body = body;

                block
                    .stmts
                    .push(syn::Stmt::Expr(syn::Expr::ForLoop(for_), None));
            }
            _ => todo!(),
        }
    }
    let block = block.stmts;
    Ok(syn::parse_quote! {
        { #(#block)* }
    })
}

fn gen_expr_tmp(
    block: &mut BlockBuilder,
    expr: Expr,
    type_of_ident: &impl Fn(&str) -> Result<Type>,
) -> Result<(syn::Expr, Option<Type>)> {
    let tmp = |block: &mut BlockBuilder, inner: syn::Expr| {
        let var = block.tmp();
        let stmt = syn::parse_quote! { let #var = #inner; };
        block.stmts.push(stmt);
        syn::parse_quote! { #var }
    };

    let (result, ty): (syn::Expr, _) = match expr {
        Expr::Int(int) => (syn::parse_quote! { #int }, None),
        Expr::Ident(identifier) => {
            let identifier = ident(&identifier);
            (syn::parse_quote! { #identifier }, None)
        }
        Expr::Index { lhs, idx } => {
            let (identifier, method, lane_idx, ty) = gen_idx("get", *lhs, *idx, type_of_ident)?;
            let expr = tmp(
                block,
                syn::parse_quote! { self.#method(#identifier, #lane_idx) },
            );
            (expr, Some(Type::Scalar { elemty: ty.elem }))
        }
        Expr::Range { .. } => todo!(),
        Expr::Call { function, args } => {
            let (args, arg_tys): (Vec<_>, Vec<_>) = args
                .into_iter()
                .map(|arg| gen_expr_tmp(block, arg, type_of_ident))
                .collect::<Result<Vec<(syn::Expr, _)>>>()?
                .into_iter()
                .unzip();

            let argtype = arg_tys
                .into_iter()
                .map(|argty| argty.unwrap().rust_type())
                .collect::<Vec<_>>()
                .join("_");

            let function = if POLYMORPHIC_FNS.contains(&function.as_str()) {
                format!("{function}_{argtype}")
            } else {
                function
            };

            let function = ident(&heck::ToSnekCase::to_snek_case(function.as_str()));
            let expr = tmp(block, syn::parse_quote! { self.#function( #(#args),* ) });
            (expr, None)
        }
        Expr::BinaryOp { op, lhs, rhs } => {
            let (lhs, lhs_ty) = gen_expr_tmp(block, *lhs, type_of_ident)?;
            let (rhs, rhs_ty) = gen_expr_tmp(block, *rhs, type_of_ident)?;

            if lhs_ty != rhs_ty {
                bail!("binary op with type mistmatch: {lhs_ty:?} != {rhs_ty:?}");
            }

            let expr = match &lhs_ty {
                // probably a rust primitive operation
                // this is extremely wonky....., but rustc typeck will complain if we get this wrong
                None => {
                    let token = match op {
                        BinaryOpKind::Add => quote::quote! { + },
                        BinaryOpKind::Mul => quote::quote! { * },
                    };
                    syn::parse_quote! { ( #lhs #token #rhs ) }
                }
                Some(_ty) => {
                    let prefix = match op {
                        BinaryOpKind::Add => "add",
                        BinaryOpKind::Mul => "mul",
                    };

                    // TODO: EXTEND somehow possibly??? ugh.

                    //let ty = ty.expect_scalar();
                    //let method = ident(&format!(
                    //    "ext_{}_u64",
                    //    ty.rust_type(),
                    //    if ty.is_signed { "s" } else { "u" }
                    //));
                    //let lhs_ext = tmp(block, syn::parse_quote! { self.#method(#lhs) });
                    //let rhs_ext = tmp(block, syn::parse_quote! { self.#method(#rhs) });

                    let method = ident(&format!("{prefix}_64"));
                    tmp(block, syn::parse_quote! { self.#method(#lhs, #rhs) })
                }
            };

            (expr, lhs_ty)
        }
    };
    Ok((result, ty))
}

fn parse_op(intr: &Intrinsic) -> Result<Vec<Stmt>> {
    let Some(operation) = &intr.operation else {
        bail!("intrinsic {} has no operation", intr.name);
    };
    let stmts = crate::parse::parse_operation(&operation.value).wrap_err("parsing intrinsic")?;
    Ok(stmts)
}

fn ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, proc_macro2::Span::call_site())
}
fn ident_opt_s(s: &Option<String>) -> Result<syn::Ident> {
    let s = s.as_deref().ok_or_eyre("missing")?;
    Ok(ident(s))
}

fn signature(intr: &Intrinsic, body: syn::Block) -> Result<syn::TraitItem> {
    let name = ident(&intr.name);

    let ret_name = ident_opt_s(&intr.ret.varname)?;
    let ret_ty = ident(map_type_to_rust_unsigned(intr.ret.r#type.as_ref().unwrap()));

    let args = [
        syn::parse_quote! {  &mut self  },
        syn::parse_quote! { #ret_name: &mut Self::#ret_ty },
    ]
    .into_iter()
    .chain(intr.parameter.iter().map(|param| -> syn::FnArg {
        let varname = ident_opt_s(&param.varname).unwrap();
        let ty = ident(map_type_to_rust_unsigned(param.r#type.as_ref().unwrap()));

        syn::parse_quote! { #varname: Self::#ty }
    }));

    Ok(syn::parse_quote! { fn #name(#(#args),*) #body })
}

fn signature_soft_arch(intr: &Intrinsic, body: syn::Block) -> Result<syn::ItemFn> {
    let name = ident(&intr.name);

    let args = intr.parameter.iter().map(|param| -> syn::FnArg {
        let varname = ident_opt_s(&param.varname).unwrap();
        let ty = ident(map_type_to_rust(param.r#type.as_ref().unwrap()));

        syn::parse_quote! { #varname: #ty }
    });

    let ret_ty = ident(map_type_to_rust(intr.ret.r#type.as_ref().unwrap()));

    Ok(syn::parse_quote! { pub fn #name( #(#args),* ) -> #ret_ty #body })
}

fn map_type_to_rust(ty: &str) -> &str {
    match ty {
        "__m128i" => ty,
        "char" => "i8",
        "short" => "i16",
        "int" => "i32",
        "__int64" => "i64",
        ty => panic!("unknown type: {ty}"),
    }
}

fn map_type_to_rust_unsigned(ty: &str) -> &str {
    match ty {
        "__m128i" => ty,
        "char" => "u8",
        "short" => "u16",
        "int" => "u32",
        "__int64" => "u64",
        ty => panic!("unknown type: {ty}"),
    }
}
