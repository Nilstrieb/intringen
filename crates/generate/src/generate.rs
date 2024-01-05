use crate::{
    parse::{Expr, Stmt},
    Intrinsic,
};
use eyre::{bail, Context, OptionExt, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn generate(intrinsics: &[Intrinsic]) -> Result<()> {
    println!("impl<C: super::Core> Intrinsics for C {{}}");
    println!("pub trait Intrinsics: super::Core {{");
    for intr in intrinsics {
        generate_intr(intr).wrap_err_with(|| format!("generating `{}`", intr.name))?;
    }

    println!("}}");

    println!();
    generate_soft_arch_module(intrinsics).wrap_err("generating soft_arch module")?;

    generate_test_module(intrinsics).wrap_err("generating test module")?;

    Ok(())
}

fn generate_soft_arch_module(intrinsics: &[Intrinsic]) -> Result<()> {
    println!("pub mod soft_arch {{");
    println!("    pub use super::super::soft_arch_types::*;");
    println!("    use super::Intrinsics;");

    for intr in intrinsics {
        generate_intr_soft_arch_wrap(intr)
            .wrap_err_with(|| format!("generating soft_arch `{}`", intr.name))?;
    }

    println!("}}");
    Ok(())
}

fn generate_test_module(intrinsics: &[Intrinsic]) -> Result<()> {
    println!("#[cfg(all(test, target_arch = \"x86_64\"))]");
    println!("pub mod tests {{");
    println!("    use super::super::compare_test_helper::hard_soft_same_128;");
    let rng = &mut SmallRng::seed_from_u64(44);

    for intr in intrinsics {
        generate_intr_test(intr, rng)
            .wrap_err_with(|| format!("generating soft_arch `{}`", intr.name))?;
    }

    println!("}}");
    Ok(())
}

fn generate_intr(intr: &Intrinsic) -> Result<(), eyre::Error> {
    eprintln!("generating {}...", intr.name);
    let signature = signature(intr)?;
    println!("    {signature} {{");
    let body = generate_body(intr).wrap_err("generating body")?;
    println!("{}", indent(&body, 8));
    println!("    }}");
    Ok(())
}

fn generate_intr_soft_arch_wrap(intr: &Intrinsic) -> Result<(), eyre::Error> {
    eprintln!("generating soft_arch wrapper {}...", intr.name);
    let signature = signature_soft_arch(intr)?;
    println!("    {signature} {{");
    let body = generate_body_soft_arch(intr).wrap_err("generating body")?;
    println!("{}", indent(&body, 8));
    println!("    }}");
    Ok(())
}

fn generate_intr_test(intr: &Intrinsic, rng: &mut SmallRng) -> Result<(), eyre::Error> {
    eprintln!("generating test {}...", intr.name);
    println!("    #[test]");
    let name = &intr.name;
    println!("    fn {name}() {{");
    // TODO: non-128
    println!("        hard_soft_same_128! {{");
    let body = generate_body_test(intr, rng).wrap_err("generating body")?;
    println!("{}", indent(&body, 12));
    println!("        }}");
    println!("    }}");
    Ok(())
}

fn generate_body_soft_arch(intr: &Intrinsic) -> Result<String> {
    let mut rust_stmts = Vec::<String>::new();

    rust_stmts.push("let mut output = unsafe { std::mem::zeroed() };".into());

    let name = &intr.name;
    let args = intr
        .parameter
        .iter()
        .map(|param| param.varname.as_deref().ok_or_eyre("parameter has no name"))
        .collect::<Result<Vec<_>>>()?
        .join(", ");
    rust_stmts.push(format!(
        "super::super::ValueCore.{name}(&mut output, {args});"
    ));

    rust_stmts.push("output".into());

    Ok(rust_stmts.join("\n"))
}

fn generate_body_test(intr: &Intrinsic, rng: &mut SmallRng) -> Result<String> {
    let mut rust_stmts = Vec::<String>::new();

    let args = intr
        .parameter
        .iter()
        .map(|param| -> Result<&str> {
            let name = param.varname.as_deref().unwrap();
            let ty = map_type_to_rust(param.r#type.as_deref().unwrap());

            rust_stmts.push(format!("let {name} = {};", random_value(ty, rng)?));
            Ok(name)
        })
        .collect::<Result<Vec<_>>>()
        .wrap_err("preparing arguments")?
        .join(", ");

    let name = &intr.name;
    rust_stmts.push(format!("{name}({args})"));

    Ok(rust_stmts.join("\n"))
}

fn random_value(ty: &str, rng: &mut SmallRng) -> Result<String> {
    Ok(match ty {
        "i16" => rng.gen::<i16>().to_string(),
        "__m128i" => format!(
            "_mm_setr_epi16({}, {}, {}, {}, {}, {}, {}, {})",
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
            rng.gen::<i16>().to_string(),
        ),
        _ => bail!("unknown type: {ty}"),
    })
}

fn indent(input: &str, indent: usize) -> String {
    let replace = format!("\n{}", " ".repeat(indent));
    let mut s = " ".repeat(indent);
    s.push_str(&input.trim_end().replace("\n", &replace));
    s
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
            "i16" => (true, 16),
            _ => bail!("unknown type: {ty}"),
        };
        let (is_signed, elem_width) = match etype {
            "SI8" => (true, 8),
            "SI16" => (true, 16),
            "UI8" => (false, 8),
            "UI16" => (false, 16),
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

fn generate_body(instr: &Intrinsic) -> Result<String> {
    let opstmts = parse_op(instr)?;
    let mut rust_stmts = Vec::<String>::new();

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

    for stmt in opstmts {
        match stmt {
            Stmt::Assign { lhs, rhs } => {
                let Expr::Index { lhs, idx } = lhs else {
                    bail!("lhs of assign must be indexing");
                };
                let Expr::Ident(ident) = *lhs else {
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

                let ty = type_of_ident(&ident)?;
                if size != ty.elem_width {
                    bail!(
                        "unsupported not-direct element indexing, size={size}, element size={}",
                        ty.elem_width
                    );
                }
                let expr = generate_expr_tmp(&mut rust_stmts, rhs, &type_of_ident)?;
                let raw = &ty.raw_type;
                let rust_type = ty.rust_type();
                let lane_idx = low / ty.elem_width;
                rust_stmts.push(format!(
                    "self.set_lane_{raw}_{rust_type}({ident}, {lane_idx}, {expr});"
                ));
            }
            _ => todo!(),
        }
    }
    Ok(rust_stmts.join("\n"))
}

fn generate_expr_tmp(
    rust_stmts: &mut Vec<String>,
    expr: Expr,
    type_of_ident: &impl Fn(&str) -> Result<VariableType>,
) -> Result<String> {
    let result = match expr {
        Expr::Int(int) => int.to_string(),
        Expr::Ident(ident) => {
            let ty = type_of_ident(&ident)?;
            if ty.is_signed != ty.rawtype_signed {
                let from = &ty.raw_type;
                let to = ty.rust_type();
                let stmt = format!("let __tmp = self.cast_sign_{from}_{to}({ident});");
                rust_stmts.push(stmt);
                "__tmp".into()
            } else {
                ident
            }
        }
        Expr::Index { lhs, idx } => {
            let Expr::Ident(ident) = *lhs else {
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

            let ty = type_of_ident(&ident)?;
            if size != ty.elem_width {
                bail!(
                    "unsupported not-direct element indexing, size={size}, element size={}",
                    ty.elem_width
                );
            }
            let raw = &ty.raw_type;
            let rust_type = ty.rust_type();
            let lane_idx = low / ty.elem_width;
            let stmt = format!("let __tmp = self.get_lane_{raw}_{rust_type}({ident}, {lane_idx});");
            rust_stmts.push(stmt);
            "__tmp".into()
        }
        Expr::Range { .. } => todo!(),
        Expr::Call { function, args } => {
            let function = heck::AsSnekCase(function);
            let args = args
                .into_iter()
                .map(|arg| generate_expr_tmp(rust_stmts, arg, type_of_ident))
                .collect::<Result<Vec<_>>>()?
                .join(", ");
            let stmt = format!("let __tmp = self.{function}({args});");
            rust_stmts.push(stmt);
            "__tmp".into()
        }
        Expr::BinaryOp { .. } => todo!(),
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

fn signature(intr: &Intrinsic) -> Result<String> {
    let name = &intr.name;

    let args = intr
        .parameter
        .iter()
        .map(|param| {
            format!(
                "{}: Self::{}",
                param.varname.as_ref().unwrap(),
                map_type_to_rust(param.r#type.as_ref().unwrap())
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let ret_name = intr.ret.varname.as_ref().unwrap();
    let ret_ty = map_type_to_rust(intr.ret.r#type.as_ref().unwrap());

    Ok(format!(
        "fn {name}(&mut self, {ret_name}: &mut Self::{ret_ty}, {args})"
    ))
}

fn signature_soft_arch(intr: &Intrinsic) -> Result<String> {
    let name = &intr.name;

    let args = intr
        .parameter
        .iter()
        .map(|param| {
            format!(
                "{}: {}",
                param.varname.as_ref().unwrap(),
                map_type_to_rust(param.r#type.as_ref().unwrap())
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let ret_ty = map_type_to_rust(intr.ret.r#type.as_ref().unwrap());

    Ok(format!("pub fn {name}({args}) -> {ret_ty}"))
}

fn map_type_to_rust(ty: &str) -> &str {
    match ty {
        "short" => "i16",
        ty => ty,
    }
}
