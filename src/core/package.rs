use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
}

impl From<&str> for Package {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

impl Package {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
