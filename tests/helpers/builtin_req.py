from typing import Any
from teo.test import TestServer, TestRequest


async def builtin_req(server: TestServer, action: str, model: str, data: Any) -> Any:
    request = TestRequest(method='POST', uri=f'/{model}/{action}', body=data)
    response = await server.process(request)
    return response.body_as_json()
