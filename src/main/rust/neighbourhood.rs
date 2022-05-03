use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
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

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_str_moore() {
        assert_eq!(Neighbourhood::from_str("M"), Ok(Neighbourhood::Moore));
    }

    #[test]
    fn test_from_str_neumann() {
        assert_eq!(Neighbourhood::from_str("N"), Ok(Neighbourhood::VonNeumann));
    }

    #[test]
    fn test_from_str_other_string() {
        assert_eq!(
            Neighbourhood::from_str("X"),
            Err(String::from("Tried to parse X as a neighbourhood type."))
        );
    }
}
