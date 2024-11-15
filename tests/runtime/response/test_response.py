from unittest import IsolatedAsyncioTestCase
from asyncio import run
from teo import TestServer
from tests.runtime.response.app import load_app


class TestResponse(IsolatedAsyncioTestCase):

    @classmethod
    def setUpClass(cls):
        async def init():
            cls.server = TestServer(load_app())
            await cls.server.setup()
        run(init())

    async def asyncSetUp(self):
        await self.server.reset()

    async def test_sample(self):
        self.assertEqual(5, 5)
        return None

    async def test_sample2(self):
        self.assertEqual(5, 5)
        return None

# test('text response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/textResponse',
#     })
#     const response = await server.process(test_request)
#     t.is(response.bodyAsString(), "foo")
# })

# test('json response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/jsonResponse',
#     })
#     const response = await server.process(test_request)
#     t.deepEqual(response.bodyAsJson(), { 'foo': 'bar' })
# })

# test('file response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/fileResponse',
#     })
#     const response = await server.process(test_request)
#     t.deepEqual(response.body(), Buffer.from('foo'))
# })

# test('cookie in text response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/textResponse',
#     })
#     const response = await server.process(test_request)
#     t.is(response.headerValue('set-cookie'), 'foo=bar')
# })

# test('cookie in json response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/jsonResponse',
#     })
#     const response = await server.process(test_request)
#     t.is(response.headerValue('set-cookie'), 'foo=bar')
# })

# test('cookie in file response', async (t) => {
#     const test_request = new TestRequest({
#         method: 'GET',
#         uri: '/fileResponse',
#     })
#     const response = await server.process(test_request)
#     t.is(response.headerValue('set-cookie'), 'foo=bar')
# })