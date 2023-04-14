from asyncio import run
from teo import App, fetch_model_class


async def async_double(n: int):
    return n * 2

async def never_valid(n: int):
  return "Never"

async def see(n: int):
  print(n)

def print_old_and_new(old: int, new: int):
  print("old is", old)
  print("new is", new)

def ppr(v):
  print(v)

async def setup():
    User = fetch_model_class("User")
    print(User.create)

async def main():
    app = App()
    app.transform("addOne", lambda x: x + 1)
    app.transform("appendX", lambda x: x + 'X')
    app.validate("neverValid", never_valid)
    app.transform("asyncDouble", async_double)
    app.compare("printOldAndNew", print_old_and_new)
    app.callback("see", see)
    app.callback("print", ppr)
    app.setup(setup)
    await app.run()

run(main())
