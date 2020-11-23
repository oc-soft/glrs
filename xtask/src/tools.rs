
use std::option::Option;
use std::process;
use std::fs;
use toml;

use super::project;
use super::install;

/// install tools to build app
pub(crate) fn install() {
    install_wasm_bindgen();
}

/// install wasm bindgen
pub(crate) fn install_wasm_bindgen() {
    let bg_path = project::local_tool_root_dir();
   
    if let Some(bg_path_str) = bg_path.to_str() {
        if let Ok(_) = fs::create_dir_all(bg_path_str) {
            let mut cargo = process::Command::new("cargo");
            cargo.arg("install")
                .arg("--root")
                .arg(bg_path_str);

            if let Some(opts) = get_wasm_bindgen_install_parameter() {
                for opt in opts {
                    cargo.arg(opt);
                }
            }
            cargo.arg("wasm-bindgen-cli")
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit());
            cargo.output()
                .expect("You could not install wasm-bindgen-cli");
            
        } 
    }
}

/// get wasm-bindgen install parameter
fn get_wasm_bindgen_install_parameter() -> Option<Vec<String>> {
    let mut cargo_path = project::root_dir();
    cargo_path.push("Cargo.toml");
    if let Ok(cargo_contents) = fs::read_to_string(cargo_path) {
        let config = cargo_contents.parse::<toml::Value>().unwrap();
        if let Some(wbg) = get_wasm_patch(&config) {
            Some(get_install_param_for_wasm_bindgen_i(wbg))
        } else {
            None
        }
    } else {
        None
    }
}

/// get wasm-bindgen patch
fn get_wasm_patch(config: &toml::Value) -> Option<&toml::Value> {
    if let Some(patch) = config.get("patch") {
        if let Some(crates_io) = patch.get("crates-io") {
            crates_io.get("wasm-bindgen") 
        } else {
            None
        }
    } else {
        None
    }

}


/// get install parameter for  wasm-bindgen 
fn get_install_param_for_wasm_bindgen_i(config: &toml::Value) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    install::add_param_if_exists(config, &mut result, "git");
    install::add_param_if_exists(config, &mut result, "version");
    install::add_param_if_exists(config, &mut result, "tag");
    install::add_param_if_exists(config, &mut result, "path");
    install::add_param_if_exists(config, &mut result, "branch");
    result 
}



// vi: se ts=4 sw=4 et:
