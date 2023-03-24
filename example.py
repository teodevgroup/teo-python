from asyncio import run
from teo import App


async def main():
    app = App()
    app.transform("addOne", lambda x: x + 1)
    await app.run()

run(main())
