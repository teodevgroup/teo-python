from asyncio import run
from teo import App


async def main():
    app = App()
    app.load("./schema.teo")
    await app.run()


run(main())
