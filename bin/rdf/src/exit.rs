// This is free and unencumbered software released into the public domain.

use clientele::SysexitsError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExitCode(pub SysexitsError);

impl core::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0.report()
    }
}

impl core::error::Error for ExitCode {}

impl From<std::boxed::Box<dyn core::error::Error>> for ExitCode {
    fn from(error: std::boxed::Box<dyn core::error::Error>) -> Self {
        std::eprintln!("rdf: {:?}", error);
        Self(SysexitsError::from(error))
    }
}

impl From<std::io::Error> for ExitCode {
    fn from(error: std::io::Error) -> Self {
        std::eprintln!("rdf: {:?}", error);
        Self(SysexitsError::from(error))
    }
}
