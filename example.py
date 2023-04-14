from asyncio import run
from teo import App


async def async_double(n: int):
    return n * 2

async def never_valid(n: int):
  return "Never"

async def see(n: int):
  print(n)

def print_old_and_new(old: int, new: int):
  print("old is", old)
  print("new is", new)

async def main():
    app = App()
    app.transform("addOne", lambda x: x + 1)
    app.transform("appendX", lambda x: x + 'X')
    app.validate("neverValid", never_valid)
    app.transform("asyncDouble", async_double)
    app.compare("printOldAndNew", print_old_and_new)
    app.callback("see", see)
    await app.run()

run(main())
