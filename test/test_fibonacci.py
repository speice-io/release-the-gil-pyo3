import speiceio_pyo3


def test_fibonacci_50():
    assert speiceio_pyo3.fibonacci_gil(50) == 12586269025
    assert speiceio_pyo3.fibonacci_nogil(50) == 12586269025
