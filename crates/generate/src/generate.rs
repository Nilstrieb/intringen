use crate::{
    parse::{Expr, Stmt},
    Intrinsic,
};
use eyre::{bail, Context, OptionExt, Result};

pub fn generate(intrinsics: &[Intrinsic]) -> Result<()> {
    println!("impl<C: super::Core> Intrinsics for C {{}}");
    println!("trait Intrinsics: super::Core {{");
    for intr in intrinsics {
        generate_intr(intr).wrap_err_with(|| format!("generating `{}`", intr.name))?;
    }

    println!("}}");

    Ok(())
}

fn generate_intr(intr: &Intrinsic) -> Result<(), eyre::Error> {
    let signature = signature(intr)?;
    println!("    {signature} {{");
    let body = generate_body(intr).wrap_err("generating body")?;
    println!("{}", indent(&body, 8));
    println!("    }}");
    Ok(())
}

fn indent(input: &str, indent: usize) -> String {
    let replace = format!("\n{}", " ".repeat(indent));
    let mut s = " ".repeat(indent);
    s.push_str(&input.trim_end().replace("\n", &replace));
    s
}

struct VariableType {
    is_signed: bool,
    elem_width: u64,
    #[allow(dead_code)]
    full_width: u64,
    raw_type: String,
}

impl VariableType {
    fn of(etype: &str, ty: &str) -> Result<Self> {
        let full_width = match ty {
            "__m128i" => 128,
            _ => bail!("unknown type: {ty}"),
        };
        let (is_signed, elem_width) = match etype {
            "SI16" => (true, 16),
            "UI8" => (false, 8),
            _ => bail!("unknown element type: {etype}"),
        };
        Ok(Self {
            is_signed,
            full_width,
            elem_width,
            raw_type: ty.to_owned(),
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
        Expr::Ident(_) => todo!(),
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
                param.r#type.as_ref().unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let ret_name = intr.ret.varname.as_ref().unwrap();
    let ret_ty = intr.ret.r#type.as_ref().unwrap();

    Ok(format!(
        "fn {name}(&mut self, {ret_name}: &mut Self::{ret_ty}, {args})"
    ))
}
