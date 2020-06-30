from setuptools import setup

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    print("Please install `setuptools-rust`")
    raise

setup(
    name="speiceio_pyo3",
    version="0.1.0",
    rust_extensions=[
        RustExtension("speiceio_pyo3.speiceio_pyo3", binding=Binding.PyO3)
    ],
    packages=["speiceio_pyo3"],
    include_package_data=True,
    zip_safe=False,
)
