use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};

#[pyclass]
#[derive(Clone)]
enum Neighbourhood {
    Moore,
    VonNeumann,
}

#[pyclass]
#[derive(Clone)]
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
        // TODO implement
        println!("{} {}", user_input, path);
        return Rules { cell: 2, range: 2, survival: 2, birth: 2, neighbourhood: Neighbourhood::Moore }
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
