from asyncio import run
from teo import App


def main():
    app = App()
    run(app.run())
