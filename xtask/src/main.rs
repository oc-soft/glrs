
use getopts::Options;
use std::env;
use std::path;
use std::result::Result;
use std::option::Option;
use std::fs;
use std::process;
use toml;

/// verb for xTask
enum Verb {
    /// display help
    Help,
    /// install some tools to build this project.
    Install,
    /// parse setting and display informantion for build
    Info,
}

/// error for xTask
enum XTaskError {
    InvalidVerb
}

/// main entry point
fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this message");
    opts.optflag("t", "setup-tools",  "setup tools to build project.");
    let args: Vec<String> = env::args().collect();
    let prg = path::Path::new(&args[0]).file_name().unwrap().to_str().unwrap();

    if args.len() > 1 {
        if let Ok(verb) = str_to_verb(&args[1]) {
            match verb {
                Verb::Help => print_usage(&prg, opts),
                Verb::Install => install_tools(),
                Verb::Info => info(),
            }
        } else {
            print_usage(&prg, opts);
        }
    } else {
        print_usage(&prg, opts);
    }
}


/// install tools to build app
fn install_tools() {
    install_wasm_bindgen();
}

/// install wasm bindgen
fn install_wasm_bindgen() {
    let mut bg_path = local_tool_bin_dir();
   
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

/// display information about this tools
fn info() {
    let mut cargo_path = project_root();
    cargo_path.push("Cargo.toml");
    if let Ok(cargo_contents) = fs::read_to_string(cargo_path) {
        let config = cargo_contents.parse::<toml::Value>().unwrap();
        if let Some(wbg) = get_wasm_patch(&config) {
            let params = get_install_param_for_wasm_bindgen_i(wbg);
            println!("{:?}", params);
        }
    }
}

/// get wasm-bindgen install parameter
fn get_wasm_bindgen_install_parameter() -> Option<Vec<String>> {
    let mut cargo_path = project_root();
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
    add_param_if_exists(config, &mut result, "git");
    add_param_if_exists(config, &mut result, "version");
    add_param_if_exists(config, &mut result, "tag");
    add_param_if_exists(config, &mut result, "path");
    add_param_if_exists(config, &mut result, "branch");
    result 
}


/// add command parameter if the option is exists in config
fn add_param_if_exists(
    config: &toml::Value,
    params: &mut Vec<String>,
    option: &str) {
    if let Some(opt_from_config) = config.get(option) {
        params.push(format!("--{}", option));
        params.push(opt_from_config.as_str().unwrap().to_string());
    }
}



/// print usage
fn print_usage(program: &str, opts: Options) {

    let brief = format!("Usage: {} [Verb] [Options]", program);
    print!("{}\n", brief);
    print!("Verb:\n");
    print!("{}\n", verb_usage());

    //print!("{}\n", opts.usage_with_format(|opt_line| {
    //    opt_line.collect::<Vec<String>>().join("\n")
    //}));
}


/// string to verb
fn str_to_verb(s: &str) -> Result<Verb, XTaskError> {
    match s {
        "help" => Ok(Verb::Help),
        "install" => Ok(Verb::Install),
        "info" => Ok(Verb::Info),
        _ => Err(XTaskError::InvalidVerb)
    }
}


/// verb to string
fn verb_to_str(v: &Verb) -> String {
    match v {
        Verb::Help => String::from("help"),
        Verb::Info => String::from("info"),
        Verb::Install => String::from("install"),
    }
}

/// verb to description
fn verb_to_description(v: &Verb) -> String {
    match v {
        Verb::Help => String::from(
            "Display this message."),
        Verb::Info => String::from(
            "Display information about tools."),
        Verb::Install => String::from(
            "install some tools to build this projects"),
    }
}

/// verb item to usage
fn verb_item_usage(v: &Verb) -> String {
    format!(
        "    {verb:<20}{desc:<52}", 
        verb = verb_to_str(&v),
        desc = verb_to_description(&v)) 
}

/// verb usage
fn verb_usage() -> String {
    let mut str_vec: Vec<String> = Vec::new();
    for v in vec![Verb::Help, Verb::Install] {
        str_vec.push(verb_item_usage(&v))
    }
    str_vec.join("\n")
}

/// get project root
fn project_root() -> path::PathBuf {
    let mut pb = path::PathBuf::from(&env!("CARGO_MANIFEST_DIR"));
    pb.pop();
    pb
}

/// get distribut directory
fn dist_dir() -> path::PathBuf {
    let mut pb = project_root();
    pb.push("target");
    pb.push("dist");
    pb
}

/// project local build directory
fn local_tool_bin_dir() -> path::PathBuf {
    let mut pb = project_root();
    pb.push("target");
    pb.push("tools");
    pb
}


// vi: se ts=4 sw=4 et:
