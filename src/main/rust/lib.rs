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
    survival: u64,
    birth: u64,
    neighbourhood: Neighbourhood
}

#[pyclass]
struct Engine {
    rules: Rules
}

#[pymethods]
impl Rules {
    #[new]
    fn new(cell: u8, range: u8, survival: u64, birth: u64, neighbourhood: Neighbourhood) -> Self {
        Rules { cell, range, survival, birth, neighbourhood }
    }

    #[staticmethod]
    fn parse(user_input: &str, path: &str) -> Self {
        if !path.is_empty() && fs::metadata(path).is_ok() {
            let json_rules = fs::read_to_string(path).unwrap();
            let rules: Rules = serde_json::from_str(&json_rules).unwrap();
            return rules;
        } else if !user_input.is_empty() {
            let values: Vec<&str> = user_input.split([';', ':']).collect();
            // "C:10;R:8;S:5;B:1;N:'M'"
            let get_rule = |rule_acronym: &str| -> u8 { values[values.iter().position(|&x| x == rule_acronym).unwrap() + 1].parse::<u8>().unwrap() };
            return Rules { 
                cell: get_rule("C"), 
                range: get_rule("R"), 
                survival: get_rule("S"), 
                birth: get_rule("B"), 
                neighbourhood: Neighbourhood::Moore 
            };
        }
        return Rules { 
            cell: 2, 
            range: 2, 
            survival: 2, 
            birth: 2, 
            neighbourhood: Neighbourhood::Moore 
        };
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new(rules: Rules) -> Self {
        Engine { rules }
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
