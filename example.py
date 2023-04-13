from asyncio import run
from teo import App


async def async_double(n: int):
    return n * 2

async def main():
    app = App()
    app.transform("addOne", lambda x: x + 1)
    app.transform("appendX", lambda x: x + 'X')
    app.transform("asyncDouble", async_double)
    await app.run()

run(main())
