from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="websocket-example",
    version="0.0.1",
    rust_extensions=[RustExtension(
        "websocket_example.websocket_example", binding=Binding.PyO3)],
    packages=["websocket_example"],
    include_package_data=True,
    install_requires=[

    ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
