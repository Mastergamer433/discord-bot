use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

macro_rules! impl_from {
    ($e:ty, $err:tt, $enum:tt) => {
        impl From<$e> for $enum {
            fn from(v: $e) -> Self {
                Self::$err(v)
            }
        }
    };
}

impl_from!(std::io::Error, Io, ConfigError);
impl_from!(serde_json::Error, Json, ConfigError);

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed loading config: {:?}", self)
    }
}

impl Error for ConfigError {}
