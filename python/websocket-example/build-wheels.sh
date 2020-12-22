#!/bin/bash


# set -ex

# curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
# export PATH="$HOME/.cargo/bin:$PATH"

# cd /io

# for PYBIN in /opt/python/cp{35,36,37,38,39}*/bin; do
#     "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
#     "${PYBIN}/python" setup.py bdist_wheel
# done


pip3 install -U setuptools wheel setuptools-rust
python3 setup.py bdist_wheel
for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done