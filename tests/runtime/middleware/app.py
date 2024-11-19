from typing import cast, Any
from teo import App, Response, Request
from tests.helpers.schema_path_args import schema_path_args


class NumberWrapper:
    def __init__(self, number: int):
        self.number = number


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def inspect(request: Request) -> Response:
        number_from_values = request.local_values().get("number")
        number_from_objects = cast(NumberWrapper, request.local_objects().get("number")).number
        return Response.teon({
            "numberFromValues": number_from_values,
            "numberFromObjects": number_from_objects
        })
    app.main_namespace().define_handler("inspect", inspect)
    def request_outer(_args: Any):
        async def middleware(request: Request, next):
            request.local_values().insert("number", 42)
            return await next(request)
        return middleware
    app.main_namespace().define_request_middleware("requestOuter", request_outer)
    def request_middle(_args: Any):
        async def middleware(request: Request, next):
            request.local_values().insert("number", cast(int, request.local_values().get("number")) * 2)
            return await next(request)
        return middleware
    app.main_namespace().define_request_middleware("requestMiddle", request_middle)
    def request_inner(_args: Any):
        async def middleware(request: Request, next):
            request.local_values().insert("number", cast(int, request.local_values().get("number")) + 16)
            return await next(request)
        return middleware
    app.main_namespace().define_request_middleware("requestInner", request_inner)
    def handler_outer(_args: Any):
        async def middleware(request: Request, next):
            request.local_objects().insert("number", NumberWrapper(24))
            return await next(request)
        return middleware
    app.main_namespace().define_handler_middleware("handlerOuter", handler_outer)
    def handler_middle(_args: Any):
        async def middleware(request: Request, next):
            wrapper = cast(NumberWrapper, request.local_objects().get("number"))
            wrapper.number *= 4
            return await next(request)
        return middleware
    app.main_namespace().define_handler_middleware("handlerMiddle", handler_middle)
    def handler_inner(_args: Any):
        async def middleware(request: Request, next):
            wrapper = cast(NumberWrapper, request.local_objects().get("number"))
            wrapper.number += 4
            return await next(request)
        return middleware
    app.main_namespace().define_handler_middleware("handlerInner", handler_inner)
    return app
