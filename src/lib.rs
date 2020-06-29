use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn fibonacci_impl(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = a + b;

    for _i in 2..n {
        a = b;
        b = c;
        // We're not particularly concerned about the actual result, just in keeping the
        // processor busy.
        c = a.overflowing_add(b).0;
    }

    c
}

#[pyfunction]
fn fibonacci_gil(n: u64) -> PyResult<u64> {
    // The GIL is implicitly held here
    Ok(fibonacci_impl(n))
}

#[pyfunction]
fn fibonacci_nogil(py: Python, n: u64) -> PyResult<u64> {
    // Explicitly release the GIL
    py.allow_threads(|| Ok(fibonacci_impl(n)))
}

#[pymodule]
fn speiceio_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fibonacci_gil))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_nogil))?;

    Ok(())
}
