from __future__ import annotations
from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadOnlyHeaderMap, ReadWriteHeaderMap, 
    HandlerMatch, RequestCtx, ObjectId, Range, OptionVariant, EnumVariant, 
    File, Pipeline, InterfaceEnumVariant, Pipeline, serve_static_files
)
from typing import TypeVar, Union, Optional
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

    @property
    def message(&self) -> str:
        object = { "code": self.code, "message": self.error_message, "errors": self.errors }
        return dumps(object)

    def __init__(self, message: str, code: Optional[int] = None, errors: Optional[dict[str, str] = None) -> None:
        self.http_message = message
        self.code = code
        self.errors = errors

    def prefixed(self, prefix: str) -> TeoException:
        slf = copy(self)
        if slf.prefixes is None:
            slf.prefixes = []
        slf.prefixes.insert(0, prefix)
        return slf
    
    def pathed(self, path: str, message: str) -> TeoException:
        slf = copy(self)
        slf.errors = { path: message }
        return slf
    
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