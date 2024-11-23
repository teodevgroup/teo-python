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
    TestServer
)
from .annotations import CapturesAnnotationMark, RequestBodyObjectAnnotationMark, TeoAnnotationMark


T = TypeVar('T')
Next = Callable[[Request], Awaitable[Response]]

Enumerable = Union[T, list[T]]

signal(SIGINT, lambda _, __: exit(0))


async def _main(self):
    await self._run()
App.run = _main


@property
def main(self) -> Namespace:
    return self.main_namespace()
App.main = main


def _extract_handler_arguments(handler_function: Callable[..., Response | Awaitable[Response]], request: Request) -> list[Any]:
    parameters = signature(handler_function).parameters
    return _extract_to_list(parameters, request, None)


def _extract_middleware_function_arguments(middleware_function: Callable[..., Awaitable[Response]], request: Request, next: Next) -> list[Any]:
    parameters = signature(middleware_function).parameters
    return _extract_to_list(parameters, request, next)


def _extract_to_list(parameters: dict[str, Parameter], request: Request, next: Optional[Next]) -> list[Any]:
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


def _define_handler(self, name: str, callable: Callable[..., Response | Awaitable[Response]], /) -> None:
    async def base_handler(request: Request) -> Response:
        coroutine_or_response = callable(*_extract_handler_arguments(callable, request))
        if iscoroutine(coroutine_or_response):
            return await coroutine_or_response
        else:
            return coroutine_or_response
    self._define_handler(name, base_handler)
Namespace.define_handler = _define_handler
HandlerGroup.define_handler = _define_handler


def _handler(self, name: str) -> Callable[[Callable[..., Response | Awaitable[Response]]], None]:
    def decorator(callable: Callable[..., Response | Awaitable[Response]]) -> None:
        self.define_handler(name, callable)
        return callable
    return decorator
Namespace.handler = _handler
HandlerGroup.handler = _handler


def _extract_middleware_creator_arguments(args: dict[str, Any]) -> dict[str, Any]:
    return args


def _define_request_middleware(self, name: str, callback: Callable[..., Callable[[Request, Next], Awaitable[Response]]], /) -> None:
    def base_callback(args: dict[str, Any]) -> Callable[[Request, Next], Awaitable[Response]]:
        extracted_args = _extract_middleware_creator_arguments(args)
        middleware_function = callback(**extracted_args)
        async def middleware(request: Request, next: Next) -> Response:
            middleware_function_arguments = _extract_middleware_function_arguments(middleware_function, request, next)
            return await middleware_function(*middleware_function_arguments)
        return middleware
    self._define_request_middleware(name, base_callback)
Namespace.define_request_middleware = _define_request_middleware


def _define_handler_middleware(self, name: str, callback: Callable[..., Callable[[Request, Next], Awaitable[Response]]], /) -> None:
    def base_callback(args: dict[str, Any]) -> Callable[[Request, Next], Awaitable[Response]]:
        extracted_args = _extract_middleware_creator_arguments(args)
        middleware_function = callback(**extracted_args)
        async def middleware(request: Request, next: Next) -> Response:
            middleware_function_arguments = _extract_middleware_function_arguments(middleware_function, request, next)
            return await middleware_function(*middleware_function_arguments)
        return middleware
    self._define_handler_middleware(name, base_callback)
Namespace.define_handler_middleware = _define_handler_middleware


def _request_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
    def decorator(callable: Callable[..., Callable[..., Awaitable[Response]]]) -> None:
        self.define_request_middleware(name, callable)
        return callable
    return decorator
Namespace.request_middleware = _request_middleware


def _handler_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
    def decorator(callable: Callable[..., Callable[..., Awaitable[Response]]]) -> None:
        self.define_handler_middleware(name, callable)
        return callable
    return decorator
Namespace.handler_middleware = _handler_middleware


class TeoException(Exception):

    error_message: str
    code: int
    errors: Optional[dict[str, str]]

    # @property
    # def message(self) -> str:
    #     object = { "code": self.code, "message": self.error_message, "errors": self.errors }
    #     return dumps(object)

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
