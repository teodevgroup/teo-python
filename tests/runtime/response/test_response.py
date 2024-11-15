from unittest import IsolatedAsyncioTestCase
from asyncio import run
from teo import TestServer, TestRequest
from tests.runtime.response.app import load_app


class TestResponse(IsolatedAsyncioTestCase):

    server: TestServer

    @classmethod
    def setUpClass(cls):
        async def init():
            cls.server = TestServer(load_app())
            await cls.server.setup()
        run(init())

    async def asyncSetUp(self):
        await self.server.reset()

    async def test_text_response(self):
        test_request = TestRequest(method="GET", uri='/textResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_string(), 'foo')

    async def test_json_response(self):
        test_request = TestRequest(method="GET", uri='/jsonResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json(), {'foo': 'bar'})

    async def test_file_response(self):
        test_request = TestRequest(method="GET", uri='/fileResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.body(), b'foo')

    async def test_cookie_in_text_response(self):
        test_request = TestRequest(method="GET", uri='/textResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.header_value('set-cookie'), 'foo=bar')

    async def test_cookie_in_json_response(self):
        test_request = TestRequest(method="GET", uri='/jsonResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.header_value('set-cookie'), 'foo=bar')

    async def test_cookie_in_file_response(self):
        test_request = TestRequest(method="GET", uri='/fileResponse')
        response = await self.server.process(test_request)
        self.assertEqual(response.header_value('set-cookie'), 'foo=bar')
