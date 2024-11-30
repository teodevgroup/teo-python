from types import GenericAlias
from typing import TypeVar, Union, Optional, Callable, Awaitable, Self, Any
from signal import signal, SIGINT
from sys import exit
from copy import copy
from asyncio.coroutines import iscoroutine
from inspect import signature, Parameter
from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadWriteHeaderMap, Cookie, Expiration,
    HandlerMatch, ObjectId, Range, OptionVariant, File, Pipeline, LocalObjects,
    LocalValues, InterfaceEnumVariant, Pipeline, TestRequest, TestResponse, 
    TestServer, PipelineCtx
)
from .annotations import CapturesAnnotationMark, RequestBodyObjectAnnotationMark, TeoAnnotationMark, ModelObjectAnnotationMark


T = TypeVar('T')
Next = Callable[[Request], Awaitable[Response]]

Enumerable = Union[T, list[T]]

signal(SIGINT, lambda _, __: exit(0))


# Extension: App

async def _app_run(self):
    await self._run()
App.run = _app_run


@property
def _app_main(self) -> Namespace:
    return self.main_namespace()
App.main = _app_main


# Extension: Arguments extractors

def _extract_arguments_arguments(args: dict[str, Any]) -> dict[str, Any]:
    return args


def _extract_handler_arguments(handler_function: Callable[..., Response | Awaitable[Response]], request: Request) -> list[Any]:
    parameters = signature(handler_function).parameters
    return _extract_from_request(parameters, request, None)


def _extract_middleware_function_arguments(middleware_function: Callable[..., Awaitable[Response]], request: Request, next: Next) -> list[Any]:
    parameters = signature(middleware_function).parameters
    return _extract_from_request(parameters, request, next)

def _extract_pipeline_item_arguments(pipeline_item_function: Callable[..., Awaitable[Any] | Any], ctx: PipelineCtx) -> list[Any]:
    parameters = signature(pipeline_item_function).parameters
    return _extract_from_pipeline_ctx(parameters, ctx)

def _extract_from_pipeline_ctx(parameters: dict[str, Parameter], ctx: PipelineCtx) -> list[Any]:
    arguments: list[Any] = []
    for name, parameter in parameters.items():
        if parameter.kind == Parameter.POSITIONAL_ONLY or parameter.kind == Parameter.POSITIONAL_OR_KEYWORD:
            if parameter.annotation == PipelineCtx:
                arguments.append(ctx)
                continue
            if parameter.annotation == Optional[Request]:
                arguments.append(ctx.request)
                continue
            elif hasattr(parameter.annotation, '__bases__'):
                if TeoAnnotationMark in parameter.annotation.__bases__:
                    arguments.append(ctx.teo)
                elif ModelObjectAnnotationMark in parameter.annotation.__bases__:
                    arguments.append(ctx.object)
                elif CapturesAnnotationMark in parameter.annotation.__orig_bases__:
                    arguments.append(ctx.captures())
                elif RequestBodyObjectAnnotationMark in parameter.annotation.__orig_bases__:
                    arguments.append(ctx.object())
                else:
                    arguments.append(ctx.object())
        else:
            raise TeoException(f"unsupported parameter extraction: {parameter}")
    return arguments

def _extract_from_request(parameters: dict[str, Parameter], request: Request, next: Optional[Next]) -> list[Any]:
    arguments: list[Any] = []
    for name, parameter in parameters.items():
        if parameter.kind == Parameter.POSITIONAL_ONLY or parameter.kind == Parameter.POSITIONAL_OR_KEYWORD:
            if name == 'next' and next is not None:
                arguments.append(next)
                continue
            if parameter.annotation == Request:
                arguments.append(request)
            elif type(parameter.annotation) == GenericAlias:
                if parameter.annotation.__origin__ == list:
                    arguments.append(request.cookies())
                elif parameter.annotation.__origin__ == dict:
                    arguments.append(request.body_object())
                else:
                    raise TeoException(f"unsupported parameter extraction: {parameter}")
            elif hasattr(parameter.annotation, '__bases__'):
                if TeoAnnotationMark in parameter.annotation.__bases__:
                    arguments.append(request.teo())
                elif CapturesAnnotationMark in parameter.annotation.__orig_bases__:
                    arguments.append(request.captures())
                elif RequestBodyObjectAnnotationMark in parameter.annotation.__orig_bases__:
                    arguments.append(request.body_object())
                else:
                    arguments.append(request.body_object())
            else:
                arguments.append(request.body_object())
        else:
            raise TeoException(f"unsupported parameter extraction: {parameter}")
    return arguments


# Extension: Namespace & HandlerGroup

def _namespace_define_handler(self, name: str, callable: Callable[..., Response | Awaitable[Response]], /) -> None:
    async def base_handler(request: Request) -> Response:
        coroutine_or_response = callable(*_extract_handler_arguments(callable, request))
        if iscoroutine(coroutine_or_response):
            return await coroutine_or_response
        else:
            return coroutine_or_response
    self._define_handler(name, base_handler)
Namespace.define_handler = _namespace_define_handler
HandlerGroup.define_handler = _namespace_define_handler

def _namespace_handler(self, name: str) -> Callable[[Callable[..., Response | Awaitable[Response]]], None]:
    def decorator(callable: Callable[..., Response | Awaitable[Response]]) -> None:
        self.define_handler(name, callable)
        return callable
    return decorator
Namespace.handler = _namespace_handler
HandlerGroup.handler = _namespace_handler

def _namespace_define_request_middleware(self, name: str, callback: Callable[..., Callable[[Request, Next], Awaitable[Response]]], /) -> None:
    def base_callback(args: dict[str, Any]) -> Callable[[Request, Next], Awaitable[Response]]:
        extracted_args = _extract_arguments_arguments(args)
        middleware_function = callback(**extracted_args)
        async def middleware(request: Request, next: Next) -> Response:
            middleware_function_arguments = _extract_middleware_function_arguments(middleware_function, request, next)
            return await middleware_function(*middleware_function_arguments)
        return middleware
    self._define_request_middleware(name, base_callback)
Namespace.define_request_middleware = _namespace_define_request_middleware

def _namespace_define_handler_middleware(self, name: str, callback: Callable[..., Callable[[Request, Next], Awaitable[Response]]], /) -> None:
    def base_callback(args: dict[str, Any]) -> Callable[[Request, Next], Awaitable[Response]]:
        extracted_args = _extract_arguments_arguments(args)
        middleware_function = callback(**extracted_args)
        async def middleware(request: Request, next: Next) -> Response:
            middleware_function_arguments = _extract_middleware_function_arguments(middleware_function, request, next)
            return await middleware_function(*middleware_function_arguments)
        return middleware
    self._define_handler_middleware(name, base_callback)
Namespace.define_handler_middleware = _namespace_define_handler_middleware

def _namespace_request_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
    def decorator(callable: Callable[..., Callable[..., Awaitable[Response]]]) -> None:
        self.define_request_middleware(name, callable)
        return callable
    return decorator
Namespace.request_middleware = _namespace_request_middleware

def _namespace_handler_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
    def decorator(callable: Callable[..., Callable[..., Awaitable[Response]]]) -> None:
        self.define_handler_middleware(name, callable)
        return callable
    return decorator
Namespace.handler_middleware = _namespace_handler_middleware

def _namespace_define_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Awaitable[Any] | Any]]) -> None:
    def base_callback(args: dict[str, Any]) -> Callable[[PipelineCtx], Awaitable[Any] | Any]:
        extracted_args = _extract_arguments_arguments(args)
        pipeline_item_function = callback(**extracted_args)
        def pipeline_item(ctx: PipelineCtx) -> Any:
            pipeline_item_function_arguments = _extract_pipeline_item_arguments(pipeline_item_function, ctx)
            return pipeline_item_function(*pipeline_item_function_arguments)
        return pipeline_item
    self._define_pipeline_item(name, base_callback)
Namespace.define_pipeline_item = _namespace_define_pipeline_item
Namespace.define_transform_pipeline_item = _namespace_define_pipeline_item

# Extension: PipelineCtx

@property
def _pipeline_ctx_value(self) -> Any:
    return self._value()
PipelineCtx.value = _pipeline_ctx_value

@property
def _pipeline_ctx_object(self) -> Any:
    return self._object()
PipelineCtx.object = _pipeline_ctx_object

@property
def _pipeline_ctx_path(self) -> list[str | int]:
    return self._path()
PipelineCtx.path = _pipeline_ctx_path

@property
def _pipeline_ctx_teo(self) -> Any:
    return self._teo()
PipelineCtx.teo = _pipeline_ctx_teo

@property
def _pipeline_ctx_request(self) -> Optional[Request]:
    return self._request()
PipelineCtx.request = _pipeline_ctx_request


# Extension: TeoException

class TeoException(Exception):

    error_message: str
    code: int
    errors: Optional[dict[str, str]]

    def __init__(self, message: str, code: int = 500, errors: Optional[dict[str, str]] = None) -> None:
        self.code = code
        self.error_message = message
        self.errors = errors

    def message_prefixed(self, prefix: str) -> Self:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message if self.errors is not None else f'{prefix}: {self.error_message}'

        retval.errors = None if self.errors is None else { k: f'{prefix}: {v}' for k, v in self.errors.items() }
        return retval

    def path_prefixed(self, prefix: str) -> Self:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message
        retval.errors = None if self.errors is None else { f'{prefix}.{k}': v for k, v in self.errors.items() }
        return retval

    def map_path(self, mapper: Callable[[str], str]) -> Self:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message
        retval.errors = None if self.errors is None else { mapper(k): v for k, v in self.errors.items() }
        return retval

    @staticmethod
    def not_found(message: str = "not found") -> 'TeoException':
        slf = TeoException(message, 404)
        return slf
    
    @staticmethod
    def invalid_request(message: str = "value is invalid") -> 'TeoException':
        slf = TeoException(message, 400)
        return slf
    
    @staticmethod
    def internal_server_error(message: str = "internal server error") -> 'TeoException':
        slf = TeoException(message, 500)
        return slf
    
    @staticmethod
    def unauthorized(message: str = "unauthorized") -> 'TeoException':
        slf = TeoException(message, 401)
        return slf
