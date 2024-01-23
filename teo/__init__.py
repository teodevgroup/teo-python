from .teo import App, Namespace, Model, Field, Relation, Property, Enum, EnumMember, Response, Request, ReadOnlyHeaderMap, ReadWriteHeaderMap, HandlerMatch, HandlerGroup, RequestCtx
from signal import signal, SIGINT
from sys import exit


signal(SIGINT, lambda _, __: exit(0))
