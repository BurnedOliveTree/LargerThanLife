use pyo3::prelude::*;
use serde::Deserialize;

#[pyclass]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Neighbourhood {
    Moore,
    VonNeumann,
}

impl Neighbourhood {
    pub fn from_str(string: &str) -> Result<Self, std::string::String> {
        match string {
            "M" => Ok(Self::Moore),
            "N" => Ok(Self::VonNeumann),
            _ => Err(format!(
                "Tried to parse {} as a neighbourhood type.",
                string
            )),
        }
    }
}
