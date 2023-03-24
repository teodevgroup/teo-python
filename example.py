from asyncio import run
from teo import App


async def main():
    app = App(True)
    await app.run()


run(main())
