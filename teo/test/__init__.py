from asyncio import run
from unittest import IsolatedAsyncioTestCase
from .. import TestServer, TestRequest, TestResponse, App


class TestCase(IsolatedAsyncioTestCase):

    server: TestServer

    @classmethod
    def load_app(cls) -> App:
        raise NotImplementedError("load_app is not implemented")

    @classmethod
    async def set_up_all(cls) -> None:
        pass

    async def set_up(self) -> None:
        pass

    @classmethod
    def setUpClass(cls):
        async def init():
            cls.server = TestServer(cls.load_app())
            await cls.server.setup()
            await cls.set_up_all()
        run(init())

    async def asyncSetUp(self):
        await super().asyncSetUp()
        await self.server.reset()
        await self.set_up()

    