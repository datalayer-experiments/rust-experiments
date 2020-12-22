[![Datalayer](https://raw.githubusercontent.com/datalayer/datalayer/main/res/logo/datalayer-25.svg?sanitize=true)](https://datalayer.io)

# Rust Python

- https://docs.wasmer.io/integrations/rust
- https://github.com/wasmerio/wasmer-python

- https://github.com/pyo3/pyo3

- https://codeburst.io/how-to-use-rust-to-extend-python-360174ee5819

```bash
pip install setuptools_rust
# cargo install pyo3
```



# websocket-example

## Build and run example : 

```
cargo build  
python setup.py develop  
python test/test.py  
```

## Development steps 

Critical path to make a python package that can use a Rust binding to a Rust lib :

### Step 1 : Rust binding
- The binding must be defined in a file named `lib.rs`
- The binding must be compiled with the following options in `Cargo.toml` :
```
[lib]
name = "websocket_example"
crate-type = ["cdylib"]
```
`crate-type` matters to build the binding in the proper lib format.
- run `cargo build` : your rust binding should compile


### Step 2 : Python package using the binding
- add the following files, as explained in [the readme of setuptools-rust ](https://github.com/PyO3/setuptools-rust) : `MANIFEST.in`, `setup.py`, `pyproject.toml`, `build-wheels.sh`
- I adapted `build-wheels.sh` to make it fit my local env but it doesn't seem to be used anyway
- Create the minimal file for a python package : `websocket_example/__init__.py` that contains only a global import like `from .websocket_example import *`
- run `python setup.py develop` : your python package should build

### Step 3 : Use your python package

- see `test/test.py`


### Additionnal note 

- I added the handling of CTRL+C to stop the websocket server. 
- Rust is fun.