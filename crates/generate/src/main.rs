mod generate;
mod parse;

use eyre::{Context, Result};
use strong_xml::XmlRead;

#[derive(Debug, XmlRead)]
#[xml(tag = "intrinsics_list")]
struct IntrinsicsList {
    #[xml(child = "intrinsic")]
    intrinsics: Vec<Intrinsic>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "intrinsic")]
struct Intrinsic {
    #[xml(attr = "name")]
    name: String,
    #[xml(child = "return")]
    ret: Return,
    #[xml(child = "parameter")]
    parameter: Vec<Parameter>,
    #[xml(child = "operation")]
    operation: Option<Operation>,
    #[xml(child = "CPUID")]
    cpuid: Vec<CpuId>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "return")]
struct Return {
    /// Element type
    #[xml(attr = "etype")]
    etype: Option<String>,
    #[xml(attr = "type")]
    r#type: Option<String>,
    #[xml(attr = "varname")]
    varname: Option<String>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "parameter")]
struct Parameter {
    /// Element type
    #[xml(attr = "etype")]
    etype: Option<String>,
    #[xml(attr = "type")]
    r#type: Option<String>,
    #[xml(attr = "varname")]
    varname: Option<String>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "operation")]
struct Operation {
    #[xml(text)]
    value: String,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "CPUID")]
struct CpuId {
    #[xml(text)]
    value: String,
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("intrinsics.xml")
        .wrap_err("unable to find intrinsics.xml, get the file from the intel intrinsics guide")?;

    let IntrinsicsList { intrinsics: list } =
        IntrinsicsList::from_str(&data).wrap_err("failed to parse intrinsics.xml")?;

    eprintln!("loaded {} intrinsics", list.len());

    let list = list
        .into_iter()
        .filter(|intr| intr.cpuid.iter().any(|cpu| !cpu.value.contains("AVX512")))
        .filter(|intr| intr.name == "_mm_packus_epi16")
        .collect::<Vec<_>>();

    eprintln!("filtered: {}", list.len());

    generate::generate(&list).wrap_err("generating rust code")?;

    Ok(())
}
