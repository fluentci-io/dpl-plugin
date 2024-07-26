use extism_pdk::*;

pub mod helpers;

#[plugin_fn]
pub fn setup(_args: String) -> FnResult<String> {
    let stdout = helpers::setup_dpl()?;
    Ok(stdout)
}
