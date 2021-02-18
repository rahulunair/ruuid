use pyo3::prelude::*;
use uuid::Uuid;

#[pymodule(ruuid)]
fn ruuid(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "uuid4")]
    // create a new random UUID
    fn uuid4(_py: Python) -> PyResult<String> {
        Ok(format!("{}", Uuid::new_v4()))
    }

    Ok(())
}
