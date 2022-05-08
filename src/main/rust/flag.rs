use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub enum Flag {
    RDefaultCell,
    RDefaultRange,
    RDefaultSurvival,
    RDefaultBirth,
    RDefaultNeighbourhood,
    RFLoadIncorrect,
    EBFLoadIncorrect
}
