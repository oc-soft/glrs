
use std::env;
use std::path;


mod build;
mod project;
mod install;
mod tools;
mod verb;
mod error;

use verb::Verb;

/// main entry point
fn main() {

    if let Some(verb_opt) = env::args().nth(1) {
        if let Ok(verb) = Verb::from(verb_opt.as_str()) {
            task(&verb); 
        } else {
            task(&Verb::Help);
        }
    } else {
        task(&Verb::Help);
    }
}




/// print usage
fn print_usage() {

    let pkg_name = project::package_name().unwrap(); 

    let arg0 = env::args().next().unwrap();

    let program = path::Path::new(&arg0).file_name().unwrap().to_str().unwrap();

    let brief = format!("Usage: {} [Verb] [Options]", program);
    print!("{}\n", brief);
    print!("Create {} assets\n", pkg_name);
    print!("Verb:\n");
    print!("{}\n", verb_usage());

}



/// verb item to usage
fn verb_item_usage(v: &Verb) -> String {
    format!(
        "    {verb:<20}{desc:<52}", 
        verb = v.to_string(),
        desc = v.to_description()) 
}

/// verb usage
fn verb_usage() -> String {
    let mut str_vec: Vec<String> = Vec::new();
    for v in vec![Verb::Help, Verb::Install, Verb::Build] {
        str_vec.push(verb_item_usage(&v))
    }
    str_vec.join("\n")
}

/// run task
fn task(verb: &Verb) {
    for dep_task in verb.depends() {
        do_task(&dep_task);
    }
    do_task(verb);
}

/// do task
fn do_task(verb: &Verb) {
    match verb {
        Verb::Help => print_usage(),
        Verb::Install => tools::install(),
        Verb::Build => build::build(),
    }
 }

// vi: se ts=4 sw=4 et:
