import asyncio
from teo import App


async def _main():
    app = App()
    await app.run()


def main():
    asyncio.run(_main())
