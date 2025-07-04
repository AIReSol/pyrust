use pyo3::{
    pyfunction, // expose a Rust function to Python
    pyclass, // expose a Rust struct to Python
    pymethods, // expose a Rust struct's methods to Python
    pymodule, // expose a Rust module to Python
    PyResult,  // return value from a Rust function
    wrap_pyfunction, // wrap a Rust function to be used in Python
};
use pyo3::prelude::{PyModule, Bound}; // Python module and Bound wrapper
use pyo3::types::PyModuleMethods; // trait providing module methods

/// Formats the sum of two numbers as string.
#[pyfunction] 
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A simple function that multiplies two numbers
#[pyfunction]
fn multiply(a: f64, b: f64) -> PyResult<f64> {
    Ok(a * b)
}

/// A function that processes a list of numbers
#[pyfunction]
fn process_list(numbers: Vec<i32>) -> PyResult<Vec<i32>> {
    let result: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    Ok(result)
}

/// A simple struct that can be used from Python
#[pyclass]
struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

#[pymethods]
impl Calculator {
    #[new]
    fn new(value: f64) -> Self {
        Calculator { value }
    }

    fn add(&mut self, other: f64) -> f64 {
        self.value += other;
        self.value
    }

    fn multiply(&mut self, other: f64) -> f64 {
        self.value *= other;
        self.value
    }

    fn reset(&mut self) {
        self.value = 0.0;
    }

    fn __str__(&self) -> String {
        format!("Calculator(value={})", self.value)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustmod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(process_list, m)?)?;
    m.add_class::<Calculator>()?;
    Ok(())
}