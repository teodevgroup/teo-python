from __future__ import annotations
from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadWriteHeaderMap, Cookie, Expiration,
    HandlerMatch, ObjectId, Range, OptionVariant, File, Pipeline, 
    InterfaceEnumVariant, Pipeline, TestRequest, TestResponse, TestServer
)
from typing import TypeVar, Union, Optional, Callable
from signal import signal, SIGINT
from sys import exit
from copy import copy
from json import dumps


T = TypeVar('T')

Enumerable = Union[T, list[T]]

async def _main(self):
    await self._run()

App.run = _main

signal(SIGINT, lambda _, __: exit(0))

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

    def message_prefixed(self, prefix: str) -> TeoException:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message if self.errors is not None else f'{prefix}: {self.error_message}'

        retval.errors = None if self.errors is None else { k: f'{prefix}: {v}' for k, v in self.errors.items() }
        return retval

    def path_prefixed(self, prefix: str) -> TeoException:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message
        retval.errors = None if self.errors is None else { f'{prefix}.{k}': v for k, v in self.errors.items() }
        return retval

    def map_path(self, mapper: Callable[[str], str]) -> TeoException:
        retval = copy(self)
        retval.code = self.code
        retval.error_message = self.error_message
        retval.errors = None if self.errors is None else { mapper(k): v for k, v in self.errors.items() }
        return retval

    @staticmethod
    def not_found(message: str = "not found") -> TeoException:
        slf = TeoException(message, 404)
        return slf
    
    @staticmethod
    def invalid_request(message: str = "value is invalid") -> TeoException:
        slf = TeoException(message, 400)
        return slf
    
    @staticmethod
    def internal_server_error(message: str = "internal server error") -> TeoException:
        slf = TeoException(message, 500)
        return slf
    
    @staticmethod
    def unauthorized(message: str = "unauthorized") -> TeoException:
        slf = TeoException(message, 401)
        return slf