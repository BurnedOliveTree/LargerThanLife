use crate::neighbourhood::Neighbourhood;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, num::ParseIntError};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Flags {
    #[pyo3(get)]
    pub d_cell: bool,
    #[pyo3(get)]
    pub d_range: bool,
    #[pyo3(get)]
    pub d_survival: bool,
    #[pyo3(get)]
    pub d_birth: bool,
    #[pyo3(get)]
    pub d_neighbourhood: bool
}


impl Default for Flags {
    fn default() -> Flags {
        Flags {
            d_cell: false,
            d_range: false,
            d_survival: false,
            d_birth: false,
            d_neighbourhood: false,
        }
    }
}


#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Range {
    #[pyo3(get)]
    pub start: u16,
    #[pyo3(get)]
    pub end: u16,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Rules {
    // TODO check
    #[pyo3(get)]
    pub cell: u8,
    #[pyo3(get)]
    pub range: usize,
    #[pyo3(get)]
    pub survival: Range,
    #[pyo3(get)]
    pub birth: Range,
    #[pyo3(get)]
    pub neighbourhood: Neighbourhood,
    #[pyo3(get)]
    pub flags: Flags,
}

trait RangeParser {
    fn from_str(&self) -> Result<Range, ParseIntError>;
}

impl RangeParser for &str {
    fn from_str(&self) -> Result<Range, ParseIntError> {
        if self.contains('-') {
            let (value1, value2) = self.split_once('-').unwrap();
            Ok(Range {
                start: value1.parse::<u16>()?,
                end: value2.parse::<u16>()?,
            })
        } else {
            Ok(Range {
                start: self.parse::<u16>()?,
                end: self.parse::<u16>()?,
            })
        }
    }
}

trait Normalizable {
    fn normalize(self, lower: Self, upper: Self) -> Self;
}

impl Normalizable for u8 {
    fn normalize(self, lower: u8, upper: u8) -> Self {
        if self < lower {
            lower
        } else if self > upper {
            upper
        } else {
            self
        }
    }
}

impl Normalizable for usize {
    fn normalize(self, lower: usize, upper: usize) -> Self {
        if self < lower {
            lower
        } else if self > upper {
            upper
        } else {
            self
        }
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            cell: 2,
            range: 1,
            survival: Range { start: 2, end: 3 },
            birth: Range { start: 3, end: 3 },
            neighbourhood: Neighbourhood::Moore,
            flags: Flags { ..Default::default() }
        }
    }
}

#[pymethods]
impl Rules {
    #[new]
    pub fn new(
        cell: u8,
        range: usize,
        survival: Range,
        birth: Range,
        neighbourhood: Neighbourhood,
    ) -> Self {
        Rules {
            cell,
            range,
            survival,
            birth,
            neighbourhood,
            flags: Flags { ..Default::default() }
        }
    }

    #[staticmethod]
    pub fn from_str(rules: &str) -> Self {
        let default_rules = Rules {
            ..Default::default()
        };
        let mut flags = Flags {
            ..Default::default()
        };
        if !rules.is_empty() {
            let values: HashMap<&str, &str> = rules
                .split(';')
                .map(|element| element.split_once(':').unwrap_or(("", "")))
                .collect();
            let get_rule = |rule_acronym: &str| -> &str { values.get(rule_acronym).unwrap_or(&"") };
            return Rules {
                cell: get_rule("C")
                    .parse::<u8>()
                    .unwrap_or_else(|_|{flags.d_cell = true; default_rules.cell})
                    .normalize(2, 255),
                range: get_rule("R")
                    .parse::<usize>()
                    .unwrap_or(default_rules.range)
                    .normalize(1, 255),
                survival: get_rule("S").from_str().unwrap_or(default_rules.survival),
                birth: get_rule("B").from_str().unwrap_or(default_rules.birth),
                neighbourhood: Neighbourhood::from_str(get_rule("N"))
                    .unwrap_or(default_rules.neighbourhood),
                flags: flags,
            };
        }
        default_rules
    }

    #[staticmethod]
    pub fn from_file(path: &str) -> Self {
        read_to_string(path)
            .and_then(|json| serde_json::from_str(&json).map_err(Into::into))
            .unwrap_or(Rules {
                ..Default::default()
            })
    }
}

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    static WAFFLE_RULES: Rules = Rules {
        cell: 2,
        range: 7,
        survival: Range {
            start: 99,
            end: 199,
        },
        birth: Range {
            start: 75,
            end: 170,
        },
        neighbourhood: Neighbourhood::Moore,
    };

    #[test]
    fn test_load_rules_from_file() {
        let path = "./res/rules/waffle.json";
        let parsed_rules = Rules::from_file(path);
        assert_eq!(parsed_rules, WAFFLE_RULES);
    }

    #[test]
    fn test_load_wrong_rules_from_from_file() {
        let path = "./res/rules/wrong.json";
        let parsed_rules = Rules::from_file(path);
        assert_eq!(
            parsed_rules,
            Rules {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_load_rules_from_not_existing_file() {
        let path = "./res/rules/404.json";
        let parsed_rules = Rules::from_file(path);
        assert_eq!(
            parsed_rules,
            Rules {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_load_rules_from_string() {
        let user_input = "C:2;R:7;S:99-199;B:75-170;N:M";
        let parsed_rules = Rules::from_str(user_input);
        assert_eq!(parsed_rules, WAFFLE_RULES);
    }

    #[test]
    fn test_load_wrong_rules_from_user_input() {
        let user_input = "C:1;R:345;S:-5;B:\"113-115\";N:6";
        let parsed_rules = Rules::from_str(user_input);
        assert_eq!(
            parsed_rules,
            Rules {
                cell: 2,
                range: 255,
                survival: Range { start: 2, end: 3 },
                birth: Range { start: 3, end: 3 },
                neighbourhood: Neighbourhood::Moore,
            }
        );
    }

    #[test]
    fn test_load_strange_user_input_for_rules() {
        let user_input = "ABC";
        let parsed_rules = Rules::from_str(user_input);
        assert_eq!(
            parsed_rules,
            Rules {
                ..Default::default()
            }
        );
    }
}
