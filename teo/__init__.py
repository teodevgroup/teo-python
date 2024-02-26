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


T = TypeVar('T')

Enumerable = Union[T, list[T]]

async def _main(self):
    await self._run()

App.run = _main

signal(SIGINT, lambda _, __: exit(0))

class TeoException(Exception):

    message: str
    code: Optional[int]
    title: Optional[str]
    errors: Optional[dict[str, str]]
    prefixes: Optional[list[str]]

    def __init__(self, message: str, code: Optional[int] = None) -> None:
        self.message = message
        self.code = code
        self.title = None
        self.errors = None
        self.prefixes = None

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
        slf.title = "NotFound"
        return slf
    
    @staticmethod
    def value_error(message: str = "value is invalid") -> TeoException:
        slf = TeoException(message, 400)
        slf.title = "ValueError"
        return slf
    
    @staticmethod
    def internal_server_error(message: str = "internal server error") -> TeoException:
        slf = TeoException(message, 500)
        slf.title = "InternalServerError"
        return slf
    
    @staticmethod
    def unauthorized(message: str = "unauthorized") -> TeoException:
        slf = TeoException(message, 401)
        slf.title = "Unauthorized"
        return slf