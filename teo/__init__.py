from types import GenericAlias
from typing import TypeVar, Union, Optional, Callable, Awaitable, Self, Any
from signal import signal, SIGINT
from sys import exit
from copy import copy
from json import dumps
from asyncio.coroutines import iscoroutine
from inspect import signature, Parameter, isclass
from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadWriteHeaderMap, Cookie, Expiration,
    HandlerMatch, ObjectId, Range, OptionVariant, File, Pipeline, LocalObjects,
    LocalValues, InterfaceEnumVariant, Pipeline, TestRequest, TestResponse, 
    TestServer
)
from .annotations import CapturesAnnotationMark, RequestBodyObjectAnnotationMark, TeoAnnotationMark


T = TypeVar('T')

Enumerable = Union[T, list[T]]

signal(SIGINT, lambda _, __: exit(0))


async def _main(self):
    await self._run()
App.run = _main


@property
def main(self) -> Namespace:
    return self.main_namespace()
App.main = main


def define_handler(self, name: str, callable: Callable[..., Response | Awaitable[Response]], /) -> None:
    parameters = signature(callable).parameters
    def extract(request: Request) -> list[Any]:
        arguments: list[Any] = []
        for parameter in parameters.values():
            if parameter.kind == Parameter.POSITIONAL_ONLY or parameter.kind == Parameter.POSITIONAL_OR_KEYWORD:
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
    async def base_handler(request: Request) -> Response:
        coroutine_or_response = callable(*extract(request))
        if iscoroutine(coroutine_or_response):
            return await coroutine_or_response
        else:
            return coroutine_or_response
    self._define_handler(name, base_handler)
Namespace.define_handler = define_handler
HandlerGroup.define_handler = define_handler


def handler(self, name: str) -> Callable[[Callable[..., Response | Awaitable[Response]]], None]:
    def decorator(callable: Callable[..., Response | Awaitable[Response]]) -> None:
        self.define_handler(name, callable)
        return callable
    return decorator
Namespace.handler = handler
HandlerGroup.handler = handler


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
