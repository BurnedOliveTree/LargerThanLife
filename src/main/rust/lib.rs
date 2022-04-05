use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};

#[pyclass]
#[derive(Clone)]
enum Neighbourhood {
    Moore,
    VonNeumann,
}

#[pyclass]
struct Game {
    cell: u8,
    range: u8,
    survival: u64,
    birth: u64,
    neighbourhood: Neighbourhood
}

#[pymethods]
impl Game {
    #[new]
    fn new(cell: u8, range: u8, survival: u64, birth: u64, neighbourhood: Neighbourhood) -> Self {
        Game { cell, range, survival, birth, neighbourhood }
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
    m.add_class::<Game>()?;
    Ok(())
}
