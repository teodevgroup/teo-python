Teo Python
==========

Run Teo server and write custom callbacks with Python.

## Installation

```sh
pip install teo
```

## Example

```python
from asyncio import run
from teo import AppBuilder


async def main():
    app_builder = AppBuilder()
    app_builder.load("./schema.teo")
    app = await app_builder.build()
    await app.run()


run(main())
```
