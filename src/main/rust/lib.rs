use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use std::fs;
use serde::Deserialize;

#[pyclass]
#[derive(Deserialize, Debug, Clone)]
enum Neighbourhood {
    Moore,
    VonNeumann,
}

#[pyclass]
#[derive(Deserialize, Debug, Clone)]
struct Rules {
    cell: u8,
    range: u8,
    survival: std::ops::Range<u64>,
    birth: std::ops::Range<u64>,
    neighbourhood: Neighbourhood
}

#[pyclass]
struct Engine {
    rules: Rules,
    board: [[f64; 600]; 600]
}

trait RangeParser {
    fn parse_range(&self) -> std::ops::Range<u64>;
}

impl RangeParser for &str {
    fn parse_range(&self) -> std::ops::Range<u64> {
        if self.contains('-') {
            let (value1, value2) = self.split_once('-').unwrap();
            return (value1.parse::<u64>().unwrap())..(value2.parse::<u64>().unwrap());
        } else {
            let value = self.parse::<u64>().unwrap();
            return value..value;
        }
    }
}

#[pymethods]
impl Rules {
    #[new]
    fn new(cell: u8, range: u8, survival: (u64, u64), birth: (u64, u64), neighbourhood: Neighbourhood) -> Self {
        Rules { cell, range, survival: survival.0..(survival.1+1), birth: birth.0..(birth.1+1), neighbourhood }
    }

    #[staticmethod]
    fn parse(user_input: &str, path: &str) -> Self {
        if !path.is_empty() && fs::metadata(path).is_ok() {
            let json_rules = fs::read_to_string(path).unwrap();
            let rules: Rules = serde_json::from_str(&json_rules).unwrap();
            return rules;
        } else if !user_input.is_empty() {
            // "C:10;R:8;S:5;B:1;N:'M'"
            let values: std::collections::HashMap<&str, &str> = user_input
                .split(';')
                .map(|element| element.split_once(':').unwrap())
                .collect();
            let get_rule = |rule_acronym: &str| -> &str { values.get(rule_acronym).unwrap() };
            return Rules { 
                cell: get_rule("C").parse::<u8>().unwrap(), 
                range: get_rule("R").parse::<u8>().unwrap(), 
                survival: get_rule("S").parse_range(), 
                birth: get_rule("B").parse_range(), 
                neighbourhood: Neighbourhood::Moore 
            };
        }
        return Rules { 
            cell: 2, 
            range: 1, 
            survival: 2..3, 
            birth: 3..3, 
            neighbourhood: Neighbourhood::Moore 
        };
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new(rules: Rules) -> Self {
        Engine { rules, board: [[0.0; 600]; 600] }
    }

    fn generate_image(&self, window_size: u16) -> Vec<Vec<u64>> {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2);
        return (0..window_size).map(|_| (0..window_size).map(|_| rng.sample(&range)).collect()).collect();
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Neighbourhood>()?;
    m.add_class::<Rules>()?;
    m.add_class::<Engine>()?;
    Ok(())
}
