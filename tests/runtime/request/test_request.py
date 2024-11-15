from os import path
from urllib3 import encode_multipart_formdata
from teo import App
from teo.test import TestRequest, TestCase
from .app import load_app


class TestRequestTest(TestCase):

    @classmethod
    def load_app(cls) -> App:
        return load_app()
    
    async def test_path(self):
        test_request = TestRequest(method="POST", uri='/', body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json()['path'], '/')

    async def test_query(self):
        test_request = TestRequest(method="POST", uri='/?foo=bar', body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json()['query'], 'foo=bar')

    async def test_content_type_from_header(self):
        test_request = TestRequest(
            method="POST", 
            uri='/?foo=bar', 
            headers={'content-type': 'application/json'}, 
            body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json()['contentTypeFromHeader'], 'application/json')

    async def test_content_type(self):
        test_request = TestRequest(
            method="POST", 
            uri='/?foo=bar', 
            headers={'content-type': 'application/json'}, 
            body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json()['contentType'], 'application/json')

    async def test_method(self):
        test_request = TestRequest(
            method="POST", 
            uri='/?foo=bar', 
            headers={'content-type': 'application/json'}, 
            body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json()['method'], 'POST')

    async def test_captures(self):
        test_request = TestRequest(method="GET", uri='/echo/foo')
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_string(), 'foo')

    async def test_combined_captures(self):
        test_request = TestRequest(method="GET", uri='/echo/foo/bar/echo')
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_string(), 'foo/bar')

    async def test_json_body(self):
        test_request = TestRequest(
            method="PATCH", 
            uri='/echo/jsonBody', 
            body={'name': 'foo', 'age': 1})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json(), {'name': 'foo', 'age': 1})

    async def test_form_body(self):
        with open(path.join(path.dirname(__file__), 'mai.jpg')) as file:
            buffer = file.buffer.read()
        body, header = encode_multipart_formdata({
            "name": "Shiranui Mai",
            "avatar": ("mai.jpg", buffer, "image/jpeg")
        })
        test_request = TestRequest(
            method="PATCH", 
            uri='/echo/formBody', 
            headers={'Content-Type': header}, 
            body=body)
        response = await self.server.process(test_request)
        response_body = response.body_as_json()
        self.assertEqual(len(response_body), 2)
        self.assertIn('name', response_body)
        self.assertIn('avatar', response_body)
        self.assertEqual(response_body['name'], 'Shiranui Mai')
        self.assertIs(response_body['avatar'].endswith('.jpg'), True)

    async def test_cookie(self):
        test_request = TestRequest(
            method="POST", 
            uri='/echo/cookie', 
            headers={'Cookie': 'a=b'}, 
            body={})
        response = await self.server.process(test_request)
        self.assertEqual(response.body_as_json(), {'cookies': [{'name': 'a', 'value': 'b'}]})
