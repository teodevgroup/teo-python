"""This module contains classes and variables for Teo web framework."""
from __future__ import annotations
from typing import TypeVar, Union, Callable, Any, Awaitable, Optional

T = TypeVar('T')

Enumerable = Union[T, list[T]]

class App:

    def __init__(self) -> None:
        """
        Create a new application. Only one can be presented in a program.
        """
        pass

    def setup(self, callback: Callable[[Any], None | Awaitable[None]], /) -> None:
        """
        Set a callback to run when server starts right after database is connected.
        """
        pass

    def program(self, name: str, callback: Callable[[Any], None | Awaitable[None]], /) -> None:
        """
        Define a custom program with `name`. The programs can be triggered with `teo run` command.
        """
        pass

    async def run(self) -> Awaitable[None]:
        """
        Run the application.
        """

    def main_namespace(self) -> Namespace:
        """
        Get the attached main namespace of the app.
        """
        pass


class Namespace:
    """
    Namespace is where handlers and models are defined.
    """

    def is_main(self) -> bool:
        """
        Whether this namespace is the main namespace.
        """
        ...

    def is_std(self) -> bool:
        """
        Whether this namespace is the standard namespace.        
        """
        ...

    def path(self) -> list[str]:
        """
        Get the namespace's path.
        """
        ...

    def namespace(self, name: str, /) -> Optional[Namespace]:
        """
        Get a child namespace by name.
        """
        ...

    def namespace_or_create(self, name: str, /) -> Namespace:
        """
        Get a child namespace by name or create a new one with name.
        """
        ...

    def namespace_at_path(self, path: list[str], /) -> Optional[Namespace]:
        """
        Get a child namespace by path.
        """
        ...

    def namespace_or_create_at_path(self, path: list[str], /) -> Namespace:
        """
        Get a child namespace by path or create a new one with path.        
        """
        ...

    def define_model_decorator(self, name: str, callback: Callable[[dict[str, Any], Model], None], /) -> None:
        """
        Define a model decorator.
        """
        ...

    def define_model_field_decorator(self, name: str, callback: Callable[[dict[str, Any], Field], None], /) -> None:
        """
        Define a model field decorator.
        """
        ...

    def define_model_relation_decorator(self, name: str, callback: Callable[[dict[str, Any], Relation], None], /) -> None:
        """
        Define a model relation decorator.
        """
        ...

    def define_model_property_decorator(self, name: str, callback: Callable[[dict[str, Any], Property], None], /) -> None:
        """
        Define a model property decorator.
        """
        ...

    def define_enum_decorator(self, name: str, callback: Callable[[dict[str, Any], Enum], None], /) -> None:
        """
        Define a enum decorator.
        """
        ...

    def define_enum_member_decorator(self, name: str, callback: Callable[[dict[str, Any], EnumMember], None], /) -> None:
        """
        Define a enum member decorator.
        """
        ...

    def define_pipeline_item(self, name: str, callback: Callable[[Any, Any, Any, Any], Any | Awaitable[Any]], /) -> None:
        """
        Define a pipeline item.
        """
        ...

    def define_transform_pipeline_item(self, name: str, callback: Callable[[Any, Any, Any, Any], Any | Awaitable[Any]], /) -> None:
        """
        Define a transform pipeline item.
        """
        ...

    def define_validator_pipeline_item(self, name: str, callback: Callable[[Any, Any, Any, Any], str | bool | None | Awaitable[str | bool | None]], /) -> None:
        """
        Define a validator pipeline item.
        """
        ...

    def define_callback_pipeline_item(self, name: str, callback: Callable[[Any, Any, Any, Any], None | Awaitable[None]], /) -> None:
        """
        Define a callback pipeline item.
        """
        ...

    def define_compare_pipeline_item(self, name: str, callback: Callable[[Any, Any, Any, Any, Any], str | bool | None | Awaitable[str | bool | None]], /) -> None:
        """
        Define a compare pipeline item.
        """
        ...

    def define_handler(self, name: str, callback: Callable[[RequestCtx], Response | Awaitable[Response]], /) -> None:
        """
        Define a handler.
        """
        ...

    def define_handler_group(self, name: str, callback: Callable[[HandlerGroup], None], /) -> None:
        """
        Define a handler group with callback.
        """
        ...

    def define_model_handler_group(self, name: str, callback: Callable[[HandlerGroup], None], /) -> None:
        """
        Define a model handler group with callback.
        """
        ...

    def define_middleware(self, name: str, callback: Callable[[Any], Callable[[RequestCtx, Callable[[RequestCtx], Awaitable[Response]]], Awaitable[Response]]], /) -> None:
        """
        Define a middleware.
        """
        ...

class HandlerGroup:
    """
    A handler group contains handlers and it takes a namespace in the request URL.
    """

    def define_handler(self, name: str, callback: Callable[[RequestCtx], Response | Awaitable[Response]], /) -> None:
        """
        Define a handler.
        """
        ...

class Model:
    """
    A model in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this model.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class Field:
    """
    A model field in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this model field.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class Relation:
    """
    A model relation in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this model relation.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class Property:
    """
    A model property in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this model property.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class Enum:
    """
    An enum in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this enum.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class EnumMember:
    """
    An enum member in Teo schema.
    """

    def set_data(self, key: str, value: Any, /) -> None:
        """
        Attach custom data with key to this enum member.
        """
        ...

    def data(self, key: str, /) -> Optional[Any]:
        """
        Get attached custom data by key.
        """
        ...

class Response:
    """
    An HTTP response.
    """

    @staticmethod
    def empty() -> Response:
        """
        Create an empty HTTP response.
        """
        ...

    @staticmethod
    def string(content: str, content_type: str, /) -> Response:
        """
        Create a string response.
        """
        ...

    @staticmethod
    def teon(value: Any, /) -> Response:
        """
        Create a Teon response.
        """
        ...

    @staticmethod
    def html(content: str, /) -> Response:
        """
        Create an HTML response.
        """
        ...

    @staticmethod
    def data(value: Any, /) -> Response:
        """
        Create a Teon response with `value` wrapped in `data` key.
        """
        ...

    @staticmethod
    def data_meta(data: Any, meta: Any, /) -> Response:
        """
        Create a Teon response with `data` and `meta` field.
        """
        ...

    @staticmethod
    def file(path: str, /) -> Response:
        """
        Create a file response with `path`.
        """
        ...

    @staticmethod
    def redirect(path: str, /) -> Response:
        """
        Create a redirect response.
        """
        ...

    def set_code(self, code: int, /) -> None:
        """
        Set the response' status code.
        """
        ...

    def code(self) -> int:
        """
        Get the response' status code.
        """
        ...

    def headers(self) -> ReadWriteHeaderMap:
        """
        Get the headers of the response.
        """
        ...

    def is_file(self) -> bool:
        """
        Whether the response body is file.
        """
        ...

    def is_text(self) -> bool:
        """
        Whether the response body is text.
        """
        ...

    def is_empty(self) -> bool:
        """
        Whether the response body is empty.
        """
        ...

    def is_teon(self) -> bool:
        """
        Whether the response body is Teon.
        """
        ...

    def get_text(self) -> Optional[str]:
        """
        Get the response text. If the response' type is not text, None is returned.
        """
        ...

    def get_teon(self) -> Optional[Any]:
        """
        Get the response Teon value. If the response' type is not Teon, None is returned.
        """
        ...

    def get_file(self) -> Optional[str]:
        """
        Get the response file path. If the response' type is not file, None is returned.
        """
        ...

class ReadWriteHeaderMap:
    """
    The readwrite headers of the response. 
    """

    def keys(self) -> list[str]:
        """
        Get the header keys.
        """
        ...

    def len(self) -> int:
        """
        Get the number of header entries in this map.
        """
        ...

    def contains_key(self, key: str, /) -> bool:
        """
        Whether the header map contains an entry by key.
        """
        ...

    def get(self, key: str, /) -> Optional[str]:
        """
        Get header value by key.
        """
        ...

    def set(self, key: str, value: str, /) -> None:
        """
        Set a header value.
        """

class Request:
    """
    An HTTP request.
    """

    def method(self) -> str:
        """
        Get the request's HTTP method.
        """
        ...

    def path(self) -> str:
        """
        Get the request's path.
        """
        ...

    def query_string(self) -> str:
        """
        Get the request's query string.
        """
        ...

    def content_type(self) -> str:
        """
        Get the request's content type.
        """
        ...

    def headers(self) -> ReadOnlyHeaderMap:
        """
        Get the request's headers.
        """

class ReadOnlyHeaderMap:
    """
    The readonly headers of the request. 
    """

    def keys(self) -> list[str]:
        """
        Get the header keys.
        """
        ...

    def len(self) -> int:
        """
        Get the number of header entries in this map.
        """
        ...

    def contains_key(self, key: str, /) -> bool:
        """
        Whether the header map contains an entry by key.
        """
        ...

    def get(self, key: str, /) -> Optional[str]:
        """
        Get header value by key.
        """
        ...

class HandlerMatch:
    """
    The request handler match result.
    """

    def path(self) -> list[str]:
        """
        Get the matched handler's path.
        """
        ...

    def handler_name(self) -> str:
        """
        Get the handler's name.
        """
        ...

    def captures(self) -> dict[str, str]:
        """
        Get the URL path captures dict.
        """
        ...

class RequestCtx:
    """
    The HTTP request context.
    """

    def request(self) -> Request:
        """
        Get the HTTP request object.
        """
        ...

    def body(self) -> Any:
        """
        Get the HTTP request's body.
        """
        ...

    def teo(self) -> Any:
        """
        Get the ORM context.
        """
        ...

    def handler_match(self) -> HandlerMatch:
        """
        Get the handler match result.
        """
        ...

    def path_arguments(self) -> Any:
        """
        Get the path arguments.
        """

    
class ObjectId:
    """
    ObjectId represents the ObjectId data type in Teo schema.
    """
    
    def to_string(self) -> str:
        """
        Convert the ObjectId object to string.
        """
        ...

    @staticmethod
    def from_string(string: str) -> ObjectId:
        """
        Create a new ObjectId from a string.
        """
        ...

class Range[T]:
    """
    A range value in Teo schema.
    """

    def upperbond(self) -> T:
        """
        Get the range's upperbond.
        """
        ...

    def lowerbond(self) -> T:
        """
        Get the range's lowerbond.
        """
        ...

    def is_closed(self) -> bool:
        """
        Whether this range is closed.
        """
        ...

    def is_open(self) -> bool:
        """
        Whether this range is open.
        """
        ...

class EnumVariant:

    def to_string(self) -> str:
        ...

    @staticmethod
    def from_string(value: str, /) -> EnumVariant:
        ...

class OptionVariant:
    pass

class InterfaceEnumVariant:
    pass

class Pipeline:
    pass

class File:
    """
    A file input in form data requests.
    """

    """
    The file path of the file.
    """
    filepath: str

    """
    The file's content type.
    """
    content_type: Optional[str]

    """
    The name of the file.
    """
    filename: str

    """
    The file name extension of the file.
    """
    filename_ext: str

def serve_static_files(base: str, path: str) -> Response:
    """
    Serve static files.
    """

class TeoException(Exception):

    message: str
    code: Optional[int]
    title: Optional[str]
    errors: Optional[dict[str, str]]
    prefixes: list[str]

    def __init__(self, message: str, code: Optional[int] = None) -> None:
        pass

    def prefixed(self, prefix: str) -> TeoException:
        pass
    
    def pathed(self, path: str, message: str) -> TeoException:
        pass
    
    @staticmethod
    def not_found(message: str = "not found") -> TeoException:
        pass
    
    @staticmethod
    def value_error(message: str = "value is invalid") -> TeoException:
        pass
    
    @staticmethod
    def internal_server_error(message: str = "internal server error") -> TeoException:
        pass
    
    @staticmethod
    def unauthorized(message: str = "unauthorized") -> TeoException:
        pass