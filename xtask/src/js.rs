
use std::fs;
use std::option::Option;
use std::process;

use super::project;
use super::package;
use json;

/// setup javascrript environment
pub(crate) fn setup() {
    copy_test_to_package();
    create_package_json();
}

/// test javascript module
pub(crate) fn test() {
    let mut cmd = process::Command::new("npm");
    cmd.arg("test");
    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit());
    let mut test_dst_dir = project::js_project_dir();
    test_dst_dir.push("test");
    cmd.current_dir(test_dst_dir); 
    cmd.output().expect("You could not launch javascritp test script.");
}

/// copy test code into package
pub(crate) fn copy_test_to_package() {
    let mut test_src_dir = project::js_template_dir();
    test_src_dir.push("test");

    let mut test_dst_dir = project::js_project_dir();
    test_dst_dir.push("test");
    if let Ok(()) =  fs::create_dir_all(test_dst_dir.clone()) {
        if let Ok(entries) = fs::read_dir(test_src_dir) {
            for dir_en_res in entries {
                if let Ok(dir_en) = dir_en_res { 
                    if dir_en.path().is_file() {
                        let mut dst_file = test_dst_dir.clone();
                        dst_file.push(dir_en.path().file_name().unwrap());
                        let _ = fs::copy(dir_en.path(), dst_file);
                    }
                }
            }
        }
    }
}

/// create package json from template and cargo toml
fn create_package_json() {
    let mut src_pkg_json_path = project::js_template_dir();
    src_pkg_json_path.push("package.json");

    let mut pkg_json_str_opt: Option<String> = None;
    if let Ok(json_contents) = fs::read_to_string(src_pkg_json_path) {
        if let Ok(pkg_json_res) = json::parse(&json_contents) {
            if let json::JsonValue::Object(mut pkg_json) = pkg_json_res {
                if let Ok(manifest_pkg) = 
                    package::Package::load_from_manifest() {
                    pkg_json.insert(
                        "version",
                        json::JsonValue::String(manifest_pkg.version()));
                    if let Some(desc) = manifest_pkg.description() {
                        pkg_json.insert(
                            "description",
                            json::JsonValue::String(desc));
                    }


                    if let Some(authors) = manifest_pkg.authors() {
                        if !authors.is_empty() {
                            pkg_json.insert(
                                "author",
                                json::JsonValue::String(authors[0].clone()));
                        }
                    }
                    if let Some(repo) = manifest_pkg.repository() {
                        pkg_json.insert(
                            "repository", 
                            json::JsonValue::String(repo));
                    }
                    if let Some(keywords) = manifest_pkg.keywords() {
                        let mut js_keywords: Vec<json::JsonValue> = Vec::new();
                        for keyword in keywords {
                            js_keywords.push(json::JsonValue::String(keyword));
                        }
                        pkg_json.insert(
                            "keywords", 
                            json::JsonValue::Array(js_keywords));
                    }
                }
                pkg_json_str_opt = Option::Some(
                    json::stringify_pretty(pkg_json, 2)); 
            }
        }
    }
    if let Some(pkg_json_str) = pkg_json_str_opt {
        let dst_dir = project::js_project_dir();
        if let Ok(()) =  fs::create_dir_all(dst_dir.clone()) {
            let mut pkg_path = dst_dir;
            pkg_path.push("package.json");
            let _ = fs::write(pkg_path, pkg_json_str);
        }
    }
}

// vi: se ts=4 sw=4 et:
