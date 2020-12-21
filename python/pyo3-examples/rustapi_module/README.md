# rustapi_module

A simple extension module built using PyO3.

## Build

```shell
python setup.py install
```
To Build on MacOS, you may need to add a `.cargo/config` file containing the following :

```
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```



## Testing

To test install tox globally and run

```shell
tox -e py
```
