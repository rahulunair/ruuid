use pyo3::prelude::*;
use uuid::Uuid;

// This module provides immutable a Universally Unique
// IDentifier (UUID) using function uuid4(),
// for generating version 4  UUID as specified in RFC 4122.
// If all you want is a unique ID, call
// uuid4() to create a random UUID.
#[pymodule(ruuid)]
fn ruuid(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "nil")]
    // generate a nil UUID
    fn nil(_py: Python) -> PyResult<String> {
        Ok(Uuid::nil().to_string())
    }
    #[pyfn(m, "uuid4")]
    // generate a new random UUID
    fn uuid4(_py: Python) -> PyResult<String> {
        Ok(Uuid::new_v4().to_string())
    }
    #[pyfn(m, "urn")]
    // generate a new random UUID in urn format
    fn urn(_py: Python) -> PyResult<String> {
        Ok(Uuid::new_v4().to_urn().to_string())
    }
    #[pyfn(m, "hyphenated")]
    // generate a new random UUID in hyphenated format
    fn hyphenated(_py: Python) -> PyResult<String> {
        Ok(Uuid::new_v4().to_hyphenated().to_string())
    }
    #[pyfn(m, "simple")]
    // generate a new random UUID in simple format
    fn simple(_py: Python) -> PyResult<String> {
        Ok(Uuid::new_v4().to_simple().to_string())
    }
    Ok(())
}
