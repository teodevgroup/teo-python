from typing import cast, Any
from teo import App, Response, Request
from tests.helpers.schema_path_args import schema_path_args


class NumberWrapper:
    def __init__(self, number: int):
        self.number = number


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def inspect(request: Request) -> Response:
        number_from_values = request.local_values["number"]
        number_from_objects = cast(NumberWrapper, request.local_objects["number"]).number
        return Response.teon({
            "numberFromValues": number_from_values,
            "numberFromObjects": number_from_objects
        })
    app.main_namespace.define_handler("inspect", inspect)
    def request_outer():
        async def middleware(request: Request, next):
            request.local_values["number"] = 42
            return await next(request) 
        return middleware
    app.main_namespace.define_request_middleware("requestOuter", request_outer)
    def request_middle(**_kwargs):
        async def middleware(request: Request, next):
            request.local_values["number"] = cast(int, request.local_values["number"]) * 2
            return await next(request)
        return middleware
    app.main_namespace.define_request_middleware("requestMiddle", request_middle)
    @app.main.request_middleware("requestInner")
    def request_inner():
        async def middleware(request: Request, next):
            request.local_values["number"] = cast(int, request.local_values["number"]) + 16
            return await next(request)
        return middleware
    def handler_outer():
        async def middleware(request: Request, next):
            request.local_objects["number"] = NumberWrapper(24)
            return await next(request)
        return middleware
    app.main_namespace.define_handler_middleware("handlerOuter", handler_outer)
    def handler_middle(**_kwargs):
        async def middleware(request: Request, next):
            wrapper = cast(NumberWrapper, request.local_objects["number"])
            wrapper.number *= 4
            return await next(request)
        return middleware
    app.main_namespace.define_handler_middleware("handlerMiddle", handler_middle)
    @app.main.handler_middleware("handlerInner")
    def handler_inner():
        async def middleware(request: Request, next):
            wrapper = cast(NumberWrapper, request.local_objects["number"])
            wrapper.number += 4
            return await next(request)
        return middleware
    return app
