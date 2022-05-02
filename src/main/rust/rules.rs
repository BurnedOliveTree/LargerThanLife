use crate::neighbourhood::Neighbourhood;

use pyo3::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, num::ParseIntError};
use tuple_transpose::TupleTranspose;

#[pyclass]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Rules {
    // TODO check
    pub cell: u8,
    pub range: usize,
    pub survival: (u16, u16),
    pub birth: (u16, u16),
    pub neighbourhood: Neighbourhood,
}

trait RangeParser {
    fn from_str(&self) -> Result<(u16, u16), ParseIntError>;
}

impl RangeParser for &str {
    fn from_str(&self) -> Result<(u16, u16), ParseIntError> {
        if self.contains('-') {
            let (value1, value2) = self.split_once('-').unwrap();
            return (value1.parse::<u16>(), value2.parse::<u16>()).transpose();
        } else {
            return (self.parse::<u16>(), self.parse::<u16>()).transpose();
        }
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            cell: 2,
            range: 1,
            survival: (2, 3),
            birth: (3, 3),
            neighbourhood: Neighbourhood::Moore,
        }
    }
}

#[pymethods]
impl Rules {
    #[new]
    fn new(
        cell: u8,
        range: usize,
        survival: (u16, u16),
        birth: (u16, u16),
        neighbourhood: Neighbourhood,
    ) -> Self {
        Rules {
            cell,
            range,
            survival,
            birth,
            neighbourhood,
        }
    }

    #[staticmethod]
    fn from_str(rules: &str) -> Self {
        let default_rules = Rules { ..Default::default() };
        if !rules.is_empty() {
            let values: HashMap<&str, &str> = rules
                .split(';')
                .map(|element| element.split_once(':').unwrap_or(("", "")))
                .collect();
            let get_rule = |rule_acronym: &str| -> &str { values.get(rule_acronym).unwrap_or(&"") };
            return Rules {
                cell: get_rule("C").parse::<u8>().unwrap_or(default_rules.cell),
                range: get_rule("R")
                    .parse::<usize>()
                    .unwrap_or(default_rules.range),
                survival: get_rule("S").from_str().unwrap_or(default_rules.survival),
                birth: get_rule("B").from_str().unwrap_or(default_rules.birth),
                neighbourhood: Neighbourhood::from_str(get_rule("N"))
                    .unwrap_or(default_rules.neighbourhood),
            };
        }
        return default_rules;
    }

    #[staticmethod]
    fn from_file(path: &str) -> Self {
        return read_to_string(path)
            .and_then(|json| serde_json::from_str(&json).map_err(Into::into))
            .unwrap_or(Rules { ..Default::default() });
    }
}

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    static WAFFLE_RULES: Rules = Rules {
        cell: 2,
        range: 7,
        survival: (99, 199),
        birth: (75, 170),
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
            Rules { ..Default::default() }
        );
    }

    #[test]
    fn test_load_rules_from_not_existing_file() {
        let path = "./res/rules/404.json";
        let parsed_rules = Rules::from_file(path);
        assert_eq!(
            parsed_rules,
            Rules { ..Default::default() }
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
        let user_input = "C:\"2\";R:345;S:-5;B:113-115;N:6";
        let parsed_rules = Rules::from_str(user_input);
        assert_eq!(
            parsed_rules,
            Rules { ..Default::default() }
        );
    }

    #[test]
    fn test_load_strange_user_input_for_rules() {
        let user_input = "ABC";
        let parsed_rules = Rules::from_str(user_input);
        assert_eq!(
            parsed_rules,
            Rules { ..Default::default() }
        );
    }
}
