from asyncio import run
from teo import App


async def main():
    app = App()
    await app.run()


run(main())
