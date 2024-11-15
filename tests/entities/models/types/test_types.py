from teo.test import TestCase, TestRequest
from teo.test.matchers import match_json_value, ignore
from .app import load_app


class TestTypes(TestCase):

    @classmethod
    def load_app(cls):
        return load_app()

    async def test_create_object(self):
        request = TestRequest(
            method='POST', 
            uri='/Support/myCreateObject',
            body={'int32': 1})
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": {
                "id": ignore,
                "int32": 1
            },
        })

    async def test_find_many_objects(self):
        request = TestRequest(
            method='POST',
            uri='/Support/myFindManyObjects',
            body={
                "orderBy": {
                    "id": "asc"
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": [],
        })
