
use super::error;

/// verb for xTask
pub(crate) enum Verb {
    /// display help
    Help,
    /// install some tools to build this project.
    Install,
    /// Build package
    Build,
}

impl Verb {

    /// get string representation
    pub fn to_string(&self) -> String {
        self::to_string(self)
    }

    /// get description
    pub fn to_description(&self) -> String {
        self::to_description(self)
    }


    /// instanciate verb from str.
    pub fn from(s: &str) -> Result<Verb, error::Error> {
        self::from(s)
    }

    /// get verbs on which self depends.
    pub fn depends(&self) -> Vec<Verb> {
        self::depends(self)
    }
}


/// verb to string
fn to_string(v: &Verb) -> String {
    match v {
        Verb::Help => String::from("help"),
        Verb::Install => String::from("install"),
        Verb::Build => String::from("build"),
    }
}

/// verb to description
fn to_description(v: &Verb) -> String {
    match v {
        Verb::Help => String::from(
            "Display this message."),
        Verb::Install => String::from(
            "install some tools to build this projects"),
        Verb::Build => String::from(
            "build package."),
    }
}

/// string to verb
fn from(s: &str) -> Result<Verb, error::Error> {
    match s {
        "help" => Ok(Verb::Help),
        "install" => Ok(Verb::Install),
        "build" => Ok(Verb::Build),
        _ => Err(error::Error::InvalidVerb)
    }
}


/// You would get dependencies for a verb.
fn depends(v: &Verb) -> Vec<Verb> {
    let mut result: Vec<Verb> = Vec::new();
    match v {
        Verb::Build => {
            result.push(Verb::Install)
        },
        _ => (),
    }
    result
}


// vi: se ts=4 sw=4 et:
