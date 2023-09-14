use ark_serialize::SerializationError;
use pyo3::PyErr;

pub fn serialisation_error_to_py_err(serialisation_error: SerializationError) -> PyErr {
    use pyo3::exceptions::{PyIOError, PyValueError};
    match serialisation_error {
        SerializationError::NotEnoughSpace => PyValueError::new_err(wrap_err_string(
            "not enough space has been allocated to serialise the object".to_string(),
        )),
        SerializationError::InvalidData => PyValueError::new_err(wrap_err_string(
            "serialised data seems to be invalid".to_string(),
        )),
        SerializationError::UnexpectedFlags => PyValueError::new_err(wrap_err_string(
            "got an unexpected flag in serialised data, check if data is malformed".to_string(),
        )),
        SerializationError::IoError(err) => PyIOError::new_err(wrap_err_string(err.to_string())),
    }
}

fn wrap_err_string(err: String) -> String {
    format!("Err From Rust: {err}")
}
