from teo.test import TestRequest, TestCase
from teo import App
from .app import load_app


class TestResponse(TestCase):

    @classmethod
    def load_app(cls) -> App:
        return load_app()

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
