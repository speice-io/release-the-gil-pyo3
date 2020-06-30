# Because `setuptools-rust` places the compile lib inside this directory, rather than at
# project root, we need to re-export the symbols.

from speiceio_pyo3.speiceio_pyo3 import *
