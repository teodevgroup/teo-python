from .teo import (
    App, Namespace, HandlerGroup, Model, Field, Relation, Property, Enum, 
    EnumMember, Response, Request, ReadOnlyHeaderMap, ReadWriteHeaderMap, 
    HandlerMatch, RequestCtx, ObjectId, Range, OptionVariant, EnumVariant, 
    File, Pipeline, InterfaceEnumVariant, Pipeline
)
from typing import TypeVar, Union
from signal import signal, SIGINT
from sys import exit


T = TypeVar('T')

Enumerable = Union[T, list[T]]

async def _main(self):
    await self._run()

App.run = _main

signal(SIGINT, lambda _, __: exit(0))
