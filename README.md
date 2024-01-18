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
from teo import App


async def main():
    app = App()
    await app.run()


run(main())
```
