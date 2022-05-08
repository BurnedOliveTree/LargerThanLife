use pyo3::prelude::*;

mod neighbourhood;

mod rules;
use rules::Rules;

mod engine;
use engine::Engine;

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Rules>()?;
    m.add_class::<Engine>()?;
    Ok(())
}
