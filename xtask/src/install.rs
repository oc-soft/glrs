
use toml;


/// add command parameter if the option is exists in config
pub(crate) fn add_param_if_exists(
    config: &toml::Value,
    params: &mut Vec<String>,
    option: &str) {
    if let Some(opt_from_config) = config.get(option) {
        params.push(format!("--{}", option));
        params.push(opt_from_config.as_str().unwrap().to_string());
    }
}


// vi: se ts=4 sw=4 et:
