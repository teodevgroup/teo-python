from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadOnlyHeaderMap, ReadWriteHeaderMap, 
    HandlerMatch, RequestCtx, ObjectId, Range, OptionVariant, EnumVariant, 
    File, Pipeline, InterfaceEnumVariant, Pipeline, serve_static_files
)
from typing import TypeVar, Union, Optional
from signal import signal, SIGINT
from sys import exit


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
    fields: Optional[dict[str, str]]
    prefixes: list[str]

    def __init__(self, message: str, code: Optional[int] = None) -> None:
        self.message = message
        self.code = code
