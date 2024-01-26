from asyncio import run
from teo import App


def main():
    app = App.with_cli(True)
    run(app.run())
