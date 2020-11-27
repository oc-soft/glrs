
use super::error;

/// verb for xTask
pub(crate) enum Verb {
    /// display help
    Help,
    /// install some tools to build this project.
    Install,
    /// Build package
    Build,
    /// setup js package
    SetupJs,
    /// test library in javascript environment.
    TestJs,
    /// clean js package
    CleanJs,
    /// publish javascript library
    PublishJs,
    /// create readme in root directory
    ReadMe,
    /// create readme in javascript publish directory
    ReadMeJs,
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
        Verb::SetupJs => String::from("setup-js"),
        Verb::PublishJs => String::from("publish-js"),
        Verb::TestJs => String::from("test-js"),
        Verb::CleanJs => String::from("clean-js"),
        Verb::ReadMe => String::from("readme"),
        Verb::ReadMeJs => String::from("readme-js"),
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
        Verb::PublishJs => String::from(
            "publish javascript library"),
        Verb::SetupJs => String::from(
            "setup javascript packages."),
        Verb::TestJs => String::from(
            "Test the library in javascript environment"),
        Verb::CleanJs => String::from(
            "Clean javascript packages."),
        Verb::ReadMe => String::from(
            "Create README.md in root directory."),
        Verb::ReadMeJs => String::from(
            "Create README.md in javascript publish directory"),
    }
}

/// string to verb
fn from(s: &str) -> Result<Verb, error::Error> {
    match s {
        "help" => Ok(Verb::Help),
        "install" => Ok(Verb::Install),
        "build" => Ok(Verb::Build),
        "setup-js" => Ok(Verb::SetupJs),
        "publish-js" => Ok(Verb::PublishJs),
        "test-js" => Ok(Verb::TestJs),
        "clean-js" => Ok(Verb::CleanJs),
        "readme" => Ok(Verb::ReadMe),
        "readme-js" => Ok(Verb::ReadMeJs),
        _ => Err(error::Error::InvalidVerb),
    }
}

/// You get dependencies for a verb.
fn depends(v: &Verb) -> Vec<Verb> {
    let mut result = Vec::new();
    depends_0(v, &mut result);
    result
}

/// You would get dependencies for a verb.
fn depends_0(v: &Verb, deps: &mut Vec<Verb>) {
    match v {
        Verb::Build => {
            deps.push(Verb::Install);
            deps.push(Verb::SetupJs);
        },
        Verb::TestJs => {
            depends_0(&Verb::Build, deps);
            deps.push(Verb::Build);
        },
        Verb::PublishJs => {
            depends_0(&Verb::TestJs, deps);
            deps.push(Verb::TestJs);
            deps.push(Verb::Build);
            deps.push(Verb::ReadMeJs);
        }
        _ => (),
    }
}


// vi: se ts=4 sw=4 et:
