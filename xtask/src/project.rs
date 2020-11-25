use std::path;
use std::fs;
use std::option::Option;
use toml;


/// package name
pub(crate) fn package_name() -> Option<String> {
    let mut cargo_path = root_dir(); 
    cargo_path.push("Cargo.toml");

    if let Ok(cargo_contents) = fs::read_to_string(cargo_path) {
        let config = cargo_contents.parse::<toml::Value>().unwrap();
        Some(config["package"]["name"].as_str().unwrap().to_string())
    } else {
        None
    }
}


/// get project root
pub(crate) fn root_dir() -> path::PathBuf {
    let mut pb = path::PathBuf::from(&env!("CARGO_MANIFEST_DIR"));
    pb.pop();
    pb
}

/// get distribut directory
#[allow(dead_code)]
pub(crate) fn dist_dir() -> path::PathBuf {
    let mut pb = root_dir();
    pb.push("target");
    pb.push("dist");
    pb
}

/// project local tool root directory
pub(crate) fn local_tool_root_dir() -> path::PathBuf {
    let mut pb = root_dir();
    pb.push("target");
    pb.push("tools");
    pb
}

/// project local tool bin directory
pub(crate) fn local_tool_bin_dir() -> path::PathBuf {
    let mut pb = local_tool_root_dir();
    pb.push("bin");
    pb
}


/// npm project directory
pub(crate) fn js_project_dir() -> path::PathBuf {
    let mut pb = root_dir();
    pb.push("target");
    pb.push("js");
    pb
}

/// javascript template directory
pub(crate) fn js_template_dir() -> path::PathBuf {
    let mut result = root_dir();
    result.push("js");
    result 
}

// vi: se ts=4 sw=4 et:
