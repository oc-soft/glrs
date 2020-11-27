
use std::fs;
use std::path;
use std::option;
use json;

use super::project;

/// create readme in root directory
pub(crate) fn create_readme_root() {
    let mut ctl_path = project::doc_src_dir();
    ctl_path.push("readme-rust.json");

    if let Ok(ctl_json_contents) = fs::read_to_string(ctl_path) {
        if let Ok(doc_ctl) = json::parse(&ctl_json_contents) {
            write_doc(
                json_as_object(&doc_ctl).expect(
                    "unexpected document control format."),
                &project::root_dir());
        }
    }
}

/// create readme in javascript directory
pub(crate) fn create_readme_js() {
    let mut ctl_path = project::doc_src_dir();
    ctl_path.push("readme-js.json");

    if let Ok(ctl_json_contents) = fs::read_to_string(ctl_path) {
        if let Ok(doc_ctl) = json::parse(&ctl_json_contents) {
            let _ = fs::create_dir_all(project::js_project_dir()); 
            write_doc(
                json_as_object(&doc_ctl).expect(
                    "unexpected document control format."),
                &project::js_project_dir());
        }
    }
}


/// write document
fn write_doc(doc_ctl: &json::object::Object, dest_dir: &path::Path) { 
    let name = doc_ctl.get("document")
            .expect("unexpected document control format").to_string();
    let toc = json_as_array(doc_ctl.get("toc")
            .expect("unexpected document control format"))
        .expect("unexpected document control format");

    let mut doc_contents: Vec<String> = Vec::new();

    for elm in toc {
        let entry = json_as_object(elm)
            .expect("unexpected document control format");

        let title = entry.get("title")
            .expect("unexpected document control format").to_string();
        let contents_ref = entry.get("contents")
            .expect("unexpected document control format").to_string();

        let mut contents_path = project::doc_src_dir();
        contents_path.push(contents_ref);
        if let Ok(contents) = fs::read_to_string(contents_path) {
            doc_contents.push(title.to_string());
            doc_contents.push(String::from(""));
            doc_contents.push(contents);
        }
    }

    let mut dest_file = path::PathBuf::new();
    dest_file.push(dest_dir); 
    dest_file.push(name);
    let _ = fs::write(dest_file, doc_contents.join("\n"));
}


/// extends json value

/// try to convert JsonValue to JsonValue::Object
pub fn json_as_object(json_value: &json::JsonValue)
    -> option::Option<&json::object::Object> {
    match *json_value {
        json::JsonValue::Object(ref value) => Some(value),
        _ => None
    }
}

/// try to convert JsonValue to JsonValue::Object
pub fn json_as_array(json_value: &json::JsonValue)
    -> option::Option<&json::Array> {
    match *json_value {
        json::JsonValue::Array(ref value) => Some(value),
        _ => None
    }
}

/// try to convert JsonValue to String
pub fn json_as_string(json_value: &json::JsonValue)
    -> option::Option<&String> {
    match *json_value {
        json::JsonValue::String(ref value) => Some(value),
        _ => None
    }
}

// vi: se ts=4 sw=4 et:
