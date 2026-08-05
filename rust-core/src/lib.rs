use pyo3::prelude::*;

#[pymodule]
mod rust_core {
    use pyo3::prelude::*;

    #[pyfunction]
    fn hello_from_bin() -> String {
        "Hello from rust!!".to_string()
    }
}
