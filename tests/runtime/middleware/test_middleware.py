from teo import App
from teo.test import TestRequest, TestCase
from .app import load_app
from teo.test.matchers import match_json_value


class TestRequestTest(TestCase):

    @classmethod
    def load_app(cls) -> App:
        return load_app()

    async def test_middleware_and_request_locals(self):
        test_request = TestRequest(method="POST", uri='/', body={})
        response = (await self.server.process(test_request)).body_as_json()
        match_json_value(response, {
            "numberFromValues": 100,
            "numberFromObjects": 100,
        })
