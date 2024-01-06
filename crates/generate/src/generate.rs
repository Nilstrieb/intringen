use crate::{
    parse::{BinaryOpKind, Expr, Stmt},
    Intrinsic,
};
use eyre::{bail, Context, OptionExt, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use syn::Block;

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

    let mut file: syn::File = syn::parse_quote! {};
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

fn generate_body_soft_arch(intr: &Intrinsic) -> Result<syn::Block> {
    let mut rust_stmts = Vec::<syn::Stmt>::new();

    rust_stmts.push(syn::parse_quote! { let mut output = unsafe { std::mem::zeroed() }; });

    let name = ident(&intr.name);

    let args = intr.parameter.iter().map(|param| -> syn::Expr {
        let name = ident_opt_s(&param.varname).unwrap();
        syn::parse_quote! { #name }
    });

    rust_stmts.push(syn::parse_quote! {
        super::super::ValueCore.#name(&mut output, #(#args),*);
    });

    rust_stmts.push(syn::Stmt::Expr(syn::parse_quote! { output }, None));

    Ok(syn::parse_quote! {
        { #(#rust_stmts)* }
    })
}

fn generate_body_test(intr: &Intrinsic, rng: &mut SmallRng) -> Result<syn::Block> {
    let mut rust_stmts = Vec::<syn::Stmt>::new();

    let args = intr
        .parameter
        .iter()
        .map(|param| -> Result<syn::Expr> {
            let name = ident_opt_s(&param.varname).unwrap();
            let ty = map_type_to_rust(param.r#type.as_deref().unwrap());

            let random = random_value(ty, rng)?;
            rust_stmts.push(syn::parse_quote! { let #name = #random; });
            Ok(syn::parse_quote! { #name })
        })
        .collect::<Result<Vec<syn::Expr>>>()
        .wrap_err("preparing arguments")?;

    let name = ident(&intr.name);
    rust_stmts.push(syn::Stmt::Expr(
        syn::parse_quote!(#name( #(#args),* )),
        None,
    ));

    Ok(syn::parse_quote! {
        { #(#rust_stmts)* }
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
        _ => bail!("unknown type: {ty}"),
    })
}

struct VariableType {
    is_signed: bool,
    rawtype_signed: bool,
    elem_width: u64,
    #[allow(dead_code)]
    full_width: u64,
    raw_type: String,
}

impl VariableType {
    fn of(etype: &str, ty: &str) -> Result<Self> {
        let (rawtype_signed, full_width) = match map_type_to_rust(ty) {
            "__m128i" => (false, 128),
            "i8" => (true, 8),
            "i16" => (true, 16),
            "i32" => (true, 32),
            "i64" => (true, 64),
            _ => bail!("unknown type: {ty}"),
        };
        let (is_signed, elem_width) = match etype {
            "SI8" => (true, 8),
            "SI16" => (true, 16),
            "SI32" => (true, 32),
            "UI8" => (false, 8),
            "UI16" => (false, 16),
            "UI32" => (false, 32),
            "UI64" => (false, 64),
            _ => bail!("unknown element type: {etype}"),
        };
        Ok(Self {
            is_signed,
            rawtype_signed,
            full_width,
            elem_width,
            raw_type: map_type_to_rust(ty).to_owned(),
        })
    }

    fn rust_type(&self) -> String {
        let pre = if self.is_signed { 'i' } else { 'u' };
        format!("{pre}{}", self.elem_width)
    }
}

fn generate_body(instr: &Intrinsic) -> Result<syn::Block> {
    let opstmts = parse_op(instr)?;

    let type_of_ident = |ident: &str| -> Result<VariableType> {
        for param in &instr.parameter {
            if param.varname.as_deref() == Some(ident) {
                return VariableType::of(
                    param.etype.as_deref().ok_or_eyre("no param etype")?,
                    param.r#type.as_deref().ok_or_eyre("no param type")?,
                );
            }
        }

        if instr.ret.varname.as_deref() == Some(ident) {
            return VariableType::of(
                instr.ret.etype.as_deref().ok_or_eyre("no param etype")?,
                instr.ret.r#type.as_deref().ok_or_eyre("no param type")?,
            );
        }

        bail!("variable {ident} not found in pseudocode");
    };

    gen_block(opstmts, &type_of_ident)
}

fn gen_idx(
    lhs: Expr,
    idx: Expr,
    type_of_ident: &impl Fn(&str) -> Result<VariableType>,
) -> Result<()> {
    let Expr::Ident(identifier) = lhs else {
        bail!("lhs of indexing must be identifier");
    };
    let Expr::Range { left, right } = idx else {
        bail!("idx argument must be range");
    };

    let ty = type_of_ident(&identifier)?;

    let (lane_idx, size): (syn::Expr, _) = match (*left, *right) {
        (Expr::Int(high), Expr::Int(low)) => {
            if high < low {
                bail!("range must be HIGH:LOW, but was {high}:{low}");
            }
            let size = high - low + 1; // (inclusive)

            let lane_idx = low / ty.elem_width;

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
            let Expr::Ident(high_ident) = *rhs else {
                bail!("rhs of lhs of + indexing must be ident");
            };
            let Expr::Int(high_offset) = *lhs else {
                bail!("lhs of lhs of + indexing must be ident");
            };

            if high_ident != low {
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
    if size != ty.elem_width {
        bail!(
            "unsupported not-direct element indexing, size={size}, element size={}",
            ty.elem_width
        );
    }
    let raw = &ty.raw_type;
    let rust_type = ty.rust_type();

    let identifier = ident(&identifier);
    let method = ident(&format!("get_lane_{raw}_{rust_type}"));
    Ok(())
}

fn gen_block(
    opstmts: Vec<Stmt>,
    type_of_ident: &impl Fn(&str) -> Result<VariableType>,
) -> Result<Block> {
    let mut rust_stmts = Vec::<syn::Stmt>::new();

    for stmt in opstmts {
        match stmt {
            Stmt::Assign {
                lhs: Expr::Index { lhs, idx },
                rhs,
            } => {
                let Expr::Ident(identifier) = *lhs else {
                    bail!("lhs of indexing must be identifier");
                };
                let Expr::Range { left, right } = *idx else {
                    bail!("idx argument must be range");
                };
                let Expr::Int(high) = *left else {
                    bail!("lhs of range must be int");
                };
                let Expr::Int(low) = *right else {
                    bail!("rhs of range must be int");
                };
                if high < low {
                    bail!("range must be HIGH:LOW, but was {high}:{low}");
                }

                let size = high - low + 1; // (inclusive)
                if !size.is_power_of_two() {
                    bail!("indexing size must be power of two");
                }

                let ty = type_of_ident(&identifier)?;
                if size != ty.elem_width {
                    bail!(
                        "unsupported not-direct element indexing, size={size}, element size={}",
                        ty.elem_width
                    );
                }
                let expr = gen_expr_tmp(&mut rust_stmts, rhs, &type_of_ident)?;
                let raw = &ty.raw_type;
                let rust_type = ty.rust_type();
                let lane_idx = low / ty.elem_width;

                let method = ident(&format!("set_lane_{raw}_{rust_type}"));
                let identifier = ident(&identifier);
                rust_stmts.push(syn::parse_quote! {
                    self.#method(#identifier, #lane_idx, #expr);
                });
            }
            Stmt::Assign {
                lhs: Expr::Ident(lhs),
                rhs,
            } => {
                let rhs = gen_expr_tmp(&mut rust_stmts, rhs, type_of_ident)?;

                let exists = type_of_ident(&lhs).is_ok();

                let lhs = ident(&lhs);
                let stmt = if exists {
                    syn::parse_quote! { #lhs = #rhs; }
                } else {
                    syn::parse_quote! { let #lhs = #rhs; }
                };

                rust_stmts.push(stmt);
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

                rust_stmts.push(syn::Stmt::Expr(syn::Expr::ForLoop(for_), None));
            }
            _ => todo!(),
        }
    }
    Ok(syn::parse_quote! {
        { #(#rust_stmts)* }
    })
}

fn gen_expr_tmp(
    rust_stmts: &mut Vec<syn::Stmt>,
    expr: Expr,
    type_of_ident: &impl Fn(&str) -> Result<VariableType>,
) -> Result<syn::Expr> {
    let tmp = |rust_stmts: &mut Vec<syn::Stmt>, inner: syn::Expr| {
        let stmt = syn::parse_quote! { let __tmp = #inner; };
        rust_stmts.push(stmt);
        syn::parse_quote! { __tmp }
    };

    let result: syn::Expr = match expr {
        Expr::Int(int) => syn::parse_quote! { #int },
        Expr::Ident(identifier) => {
            let ty = type_of_ident(&identifier);
            let identifier = ident(&identifier);
            match ty {
                Ok(ty) if ty.is_signed != ty.rawtype_signed => {
                    // intel intrinsics types kinda lie sometimes.
                    // _mm_setr_epi16 says the etype of the argument is UI16 (unsigned),
                    // while the actual type is short (signed). Do a cast to the etype, since we used that.
                    let from = &ty.raw_type;
                    let to = ty.rust_type();
                    let method = ident(&format!("cast_sign_{from}_{to}"));
                    tmp(rust_stmts, syn::parse_quote! { self.#method(#identifier) })
                }
                _ => syn::parse_quote! { #identifier },
            }
        }
        Expr::Index { lhs, idx } => {
            let Expr::Ident(identifier) = *lhs else {
                bail!("lhs of indexing must be identifier");
            };
            let Expr::Range { left, right } = *idx else {
                bail!("idx argument must be range");
            };

            let ty = type_of_ident(&identifier)?;

            let (lane_idx, size): (syn::Expr, _) = match (*left, *right) {
                (Expr::Int(high), Expr::Int(low)) => {
                    if high < low {
                        bail!("range must be HIGH:LOW, but was {high}:{low}");
                    }
                    let size = high - low + 1; // (inclusive)

                    let lane_idx = low / ty.elem_width;

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
                    let Expr::Ident(high_ident) = *rhs else {
                        bail!("rhs of lhs of + indexing must be ident");
                    };
                    let Expr::Int(high_offset) = *lhs else {
                        bail!("lhs of lhs of + indexing must be ident");
                    };

                    if high_ident != low {
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
            if size != ty.elem_width {
                bail!(
                    "unsupported not-direct element indexing, size={size}, element size={}",
                    ty.elem_width
                );
            }
            let raw = &ty.raw_type;
            let rust_type = ty.rust_type();

            let identifier = ident(&identifier);
            let method = ident(&format!("get_lane_{raw}_{rust_type}"));

            tmp(
                rust_stmts,
                syn::parse_quote! { self.#method(#identifier, #lane_idx) },
            )
        }
        Expr::Range { .. } => todo!(),
        Expr::Call { function, args } => {
            let function = ident(&heck::ToSnekCase::to_snek_case(function.as_str()));
            let args = args
                .into_iter()
                .map(|arg| gen_expr_tmp(rust_stmts, arg, type_of_ident))
                .collect::<Result<Vec<syn::Expr>>>()?;

            tmp(
                rust_stmts,
                syn::parse_quote! { self.#function( #(#args),* ) },
            )
        }
        Expr::BinaryOp { op, lhs, rhs } => {
            let lhs = gen_expr_tmp(rust_stmts, *lhs, type_of_ident)?;
            let rhs = gen_expr_tmp(rust_stmts, *rhs, type_of_ident)?;

            let token = match op {
                BinaryOpKind::Add => quote::quote! { + },
                BinaryOpKind::Mul => quote::quote! { * },
            };

            syn::parse_quote! { ( #lhs #token #rhs ) }
        }
    };
    Ok(result)
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
    let ret_ty = ident(map_type_to_rust(intr.ret.r#type.as_ref().unwrap()));

    let args = [
        syn::parse_quote! {  &mut self  },
        syn::parse_quote! { #ret_name: &mut Self::#ret_ty },
    ]
    .into_iter()
    .chain(intr.parameter.iter().map(|param| -> syn::FnArg {
        let varname = ident_opt_s(&param.varname).unwrap();
        let ty = ident(map_type_to_rust(param.r#type.as_ref().unwrap()));

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
