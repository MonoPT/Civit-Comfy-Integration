use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_civit_comfy_bindings {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
    
    #[pyfunction]
    fn start_server(port: usize, static_dir: &str, comfy_path: &str) {
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()
            .unwrap();
        
        rt.block_on(app::start_civit_frontend_server(port, static_dir, comfy_path));
    }
}
