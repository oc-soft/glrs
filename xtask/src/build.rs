
use std::process;
use std::fs;
use std::env;
use std::path;
use super::project;


/// build all assets
pub(crate) fn build() {
    library();
    js_binding()
}

/// build wasm libray 
pub(crate) fn library() {
    let mut manifest_path = project::root_dir();
    manifest_path.push("Cargo.toml");
    let mut cmd = process::Command::new("cargo");

    cmd.arg("build")
        .arg("--lib")
        .arg("--manifest-path")
        .arg(manifest_path.to_str().unwrap())
        .arg("--target")
        .arg("wasm32-unknown-unknown");

    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit());

    cmd.output().expect("You could not create web-assembly library.");
}

/// build javascript binding
pub(crate) fn js_binding() {
    let js_proj_dir = project::js_project_dir();
    
    if let Ok(_) = fs::create_dir_all(js_proj_dir.clone()) {
        let mut wbg_path = project::local_tool_bin_dir();
        wbg_path.push("wasm-bindgen");
        let pkg_wasm_path = wasm_lib_out();
        let mut cmd = process::Command::new(wbg_path.to_str().unwrap());
        cmd.arg("--out-dir")
            .arg(js_proj_dir.to_str().unwrap())
            .arg("--target")
            .arg("web-bundler")
            .arg(pkg_wasm_path);


        cmd.stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit());
        cmd.output().expect(
            "You could not create javascript assets"); 
    }
}

/// wasm library output
pub(crate) fn wasm_lib_out() -> path::PathBuf {
    let mut result = project::root_dir();
    result.push("target");
    result.push("wasm32-unknown-unknown");
    if let Ok(val) = env::var("PROFILE") {
        result.push(val);
    } else {
        result.push("debug");
    }
    result.push(project::package_name().unwrap());
    result.set_extension("wasm");
    result
}
// vi: se ts=4 sw=4 et:
