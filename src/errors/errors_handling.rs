// errors.rs
use oracle::Error as OracleError;
use std::env;
use std::error::Error;
use std::fmt;

pub trait FromEnvVarError: Error {
    fn from_env_var_error(err: env::VarError) -> Self;
}

#[derive(Debug)]
pub struct AppOracleError {
    msg: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for AppOracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AppOracleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl FromEnvVarError for AppOracleError {
    fn from_env_var_error(err: env::VarError) -> Self {
        AppOracleError {
            msg: format!("Error retrieving environment variable: {}", err),
            source: None,
        }
    }
}

impl From<OracleError> for AppOracleError {
    fn from(err: OracleError) -> Self {
        AppOracleError {
            msg: format!("OracleError: {}", err),
            source: Some(Box::new(err)),
        }
    }
}
