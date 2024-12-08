"""This module contains classes and variables for Teo web framework."""
from typing import TypeVar, Union, Callable, Any, Awaitable, Optional, Literal
from datetime import datetime

T = TypeVar('T')
Next = Callable[[Request], Awaitable[Response]]

Enumerable = Union[T, list[T]]

class App:

    def __init__(self, argv: Optional[list[str]]) -> None:
        """
        Create a new application. Only one can be presented in a program.
        """
        ...

    def setup(self, callback: Callable[[Any], None | Awaitable[None]], /) -> None:
        """
        Set a callback to run when server starts right after database is connected.
        """
        ...

    def program(self, name: str, desc: Optional[str], callback: Callable[[Any], None | Awaitable[None]], /) -> None:
        """
        Define a custom program with `name`. The programs can be triggered with `teo run` command.
        """
        ...

    async def run(self) -> Awaitable[None]:
        """
        Run the application.
        """
        ...

    @property
    def main_namespace(self) -> Namespace:
        """
        Get the attached main namespace of the app.
        """
        ...

    @property
    def main(self) -> Namespace:
        """
        Get the attached main namespace of the app.
        """
        ...


class Namespace:
    """
    Namespace is where handlers and models are defined.
    """

    @property
    def is_main(self) -> bool:
        """
        Whether this namespace is the main namespace.
        """
        ...

    @property
    def is_std(self) -> bool:
        """
        Whether this namespace is the standard namespace.        
        """
        ...

    @property
    def path(self) -> list[str]:
        """
        Get the namespace's path.
        """
        ...

    def namespace(self, name: str, /) -> Namespace:
        """
        Get a child namespace by name or create a new one with name.
        """
        ...

    def child_namespace(self, name: str, /) -> Optional[Namespace]:
        """
        Get a child namespace by name.
        """
        ...

    def child_namespace_or_create(self, name: str, /) -> Namespace:
        """
        Get a child namespace by name or create a new one with name.
        """
        ...

    def descendant_namespace_at_path(self, path: list[str], /) -> Optional[Namespace]:
        """
        Get a child namespace by path.
        """
        ...

    def descendant_namespace_or_create_at_path(self, path: list[str], /) -> Namespace:
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

    def define_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Any | Awaitable[Any]]]) -> None:
        """
        Define a pipeline item.
        """
        ...

    def pipeline_item(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Any] | Any]]], None]:
        """
        Define a pipeline item with decorator.
        """
        ...

    def pipeline_item_function(self, name: str) -> Callable[[Callable[..., Any | Awaitable[Any]]], None]:
        """
        Define a pipeline item without creator.
        """
        ...

    def define_transform_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Any | Awaitable[Any]]]) -> None:
        """
        Define a transform pipeline item.
        """
        ...

    def transform_pipeline_item(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Any] | Any]]], None]:
        """
        Define a transform pipeline item with decorator.
        """
        ...

    def transform_pipeline_item_function(self, name: str) -> Callable[[Callable[..., Any | Awaitable[Any]]], None]:
        """
        Define a transform pipeline item without creator.
        """
        ...


    def define_validator_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Any | Awaitable[Any]]]) -> None:
        """
        Define a validator pipeline item.
        """
        ...

    def validator_pipeline_item(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Any] | Any]]], None]:
        """
        Define a validator pipeline item with decorator.
        """
        ...

    def validator_pipeline_item_function(self, name: str) -> Callable[[Callable[..., Any | Awaitable[Any]]], None]:
        """
        Define a validator pipeline item without creator.
        """
        ...


    def define_callback_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Any | Awaitable[Any]]]) -> None:
        """
        Define a callback pipeline item.
        """
        ...

    def callback_pipeline_item(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Any] | Any]]], None]:
        """
        Define a callback pipeline item with decorator.
        """
        ...

    def callback_pipeline_item_function(self, name: str) -> Callable[[Callable[..., Any | Awaitable[Any]]], None]:
        """
        Define a callback pipeline item without creator.
        """
        ...        

    def define_compare_pipeline_item(self, name: str, callback: Callable[..., Callable[..., Any | Awaitable[Any]]]) -> None:
        """
        Define a compare pipeline item.
        """
        ...

    def compare_pipeline_item(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Any] | Any]]], None]:
        """
        Define a compare pipeline item with decorator.
        """
        ...

    def compare_pipeline_item_function(self, name: str) -> Callable[[Callable[..., Any | Awaitable[Any]]], None]:
        """
        Define a compare pipeline item without creator.
        """
        ...

    def define_handler(self, name: str, callback: Callable[..., Response | Awaitable[Response]], /) -> None:
        """
        Define a handler.
        """
        ...

    def handler(self, name: str) -> Callable[[Callable[..., Response | Awaitable[Response]]], None]:
        """
        Define a handler with decorator.
        """
        ...

    def define_handler_group(self, name: str, callback: Callable[[HandlerGroup], None], /) -> None:
        """
        Define a handler group with callback.
        """
        ...

    def handler_group(self, name: str) -> Optional[HandlerGroup]:
        """
        Get a defined handler group by name.
        """
        ...

    def group(self, name: str) -> HandlerGroup:
        """
        Get or create a handler group by name.
        """
        ...

    def define_model_handler_group(self, name: str, callback: Callable[[HandlerGroup], None], /) -> None:
        """
        Define a model handler group with callback.
        """
        ...

    def model_handler_group(self, name: str) -> Optional[HandlerGroup]:
        """
        Get a defined model handler group by name.
        """
        ...

    def model(self, name: str) -> HandlerGroup:
        """
        Get or create a model handler group by name.
        """
        ...

    def define_request_middleware(self, name: str, callback: Callable[..., Callable[..., Awaitable[Response]]], /) -> None:
        """
        Define a request middleware.
        """
        ...

    def define_handler_middleware(self, name: str, callback: Callable[..., Callable[..., Awaitable[Response]]], /) -> None:
        """
        Define a handler middleware.
        """
        ...
    
    def request_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
        """
        Define a request middleware with decorator.
        """
        ...

    def handler_middleware(self, name: str) -> Callable[[Callable[..., Callable[..., Awaitable[Response]]]], None]:
        """
        Define a handler middleware with decorator.
        """
        ...

class HandlerGroup:
    """
    A handler group contains handlers and it takes a namespace in the request URL.
    """

    def define_handler(self, name: str, callback: Callable[..., Response | Awaitable[Response]], /) -> None:
        """
        Define a handler.
        """
        ...

    def handler(self, name: str) -> Callable[[Callable[..., Response | Awaitable[Response]]], None]:
        """
        Define a handler with decorator.
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

class Cookies:
    """
    HTTP request or response cookies.
    """
    ...

    def append(self, cookie: Cookie) -> None:
        """
        Append a cookie into this map.
        """
        ...

    def clear(self) -> None:
        """
        Clear this cookie map.
        """
        ...

    def __iter__(self) -> CookiesIter:
        """
        Create a iterator from this cookie map.
        """
        ...


class CookiesIter:
    ...

    def __next__(self) -> Optional[Cookie]:
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
    def send_file(base: str, path: str, /) -> Response:
        """
        Serve file at `path` from `base` directory.
        """
        ...

    @staticmethod
    def redirect(path: str, /) -> Response:
        """
        Create a redirect response.
        """
        ...

    @property
    def code(self) -> int:
        """
        Get the response' status code.
        """
        ...

    @code.setter
    def set_code(self, code: int, /) -> None:
        """
        Set the response' status code.
        """
        ...

    @property
    def headers(self) -> Headers:
        """
        Get the headers of the response.
        """
        ...

    @headers.setter
    def set_headers(self, headers: Headers) -> None:
        """
        Set the headers of the response.
        """
        ...

    @property
    def is_file(self) -> bool:
        """
        Whether the response body is file.
        """
        ...

    @property
    def is_text(self) -> bool:
        """
        Whether the response body is text.
        """
        ...

    @property
    def is_empty(self) -> bool:
        """
        Whether the response body is empty.
        """
        ...

    @property
    def is_teon(self) -> bool:
        """
        Whether the response body is Teon.
        """
        ...

    @property
    def text(self) -> Optional[str]:
        """
        Get the response text. If the response' type is not text, None is returned.
        """
        ...

    @property
    def get_teon(self) -> Optional[Any]:
        """
        Get the response Teon value. If the response' type is not Teon, None is returned.
        """
        ...

    @property
    def get_file(self) -> Optional[str]:
        """
        Get the response file path. If the response' type is not file, None is returned.
        """
        ...

    @property
    def cookies(self) -> Cookies:
        """
        Get the cookies of the response.
        """
        ...

    @cookies.setter
    def set_cookies(self, cookies: Cookies) -> None:
        """
        Set the cookies of the response.
        """
        ...


class Headers:
    """
    The request or response headers. 
    """

    def keys(self) -> list[str]:
        """
        Get the header keys.
        """
        ...

    def __len__(self) -> int:
        """
        Get the number of header entries in this map.
        """
        ...

    def __contains__(self, key: str, /) -> bool:
        """
        Whether the header map contains an entry by key.
        """
        ...

    def __getitem__(self, key: str, /) -> Optional[str]:
        """
        Get header value by key.
        """
        ...

    def __setitem__(self, key: str, value: str, /) -> None:
        """
        Insert a header value.
        """
        ...

    def append(self, key: str, value: str, /) -> None:
        """
        Append a header value.
        """
        ...

    def __delitem__(self, key: str, /) -> None:
        """
        Remove a header by key.
        """
        ...

    def clear(self) -> None:
        """
        Clear the header map.
        """
        ...

    def copy(self) -> Headers:
        """
        Copy this header map.
        """    
        ...

    def __iter__(self) -> HeadersIter:
        """
        Create a iterator from this header map.
        """
        ...

class HeadersIter:
    """
    Headers iterator.
    """
    
    def __next__(self) -> tuple[str, str]:
        """
        The next pair of header name and value.
        """
        ...

SameSite = Literal["strict", "lax", "none"]


class Cookie:
    """
    The cookie.
    """

    def __init__(self, 
                 name: str, 
                 value: Optional[str] = None,
                 http_only: Optional[bool] = None,
                 secure: Optional[bool] = None,
                 same_site: Optional[SameSite] = None,
                 partitioned: Optional[bool] = None,
                 max_age: Optional[float] = None,
                 path: Optional[str] = None,
                 domain: Optional[str] = None,
                 expires: Optional[Expiration] = None) -> None:
        """
        Create a new cookie. If only one string parameter is provided, it is 
        parsed.
        """
        ...

    @property
    def name(self) -> str:
        """
        Get the cookie's name.
        """
        ...

    @name.setter
    def set_name(self, name: str) -> None:
        """
        Set the cookie's name.
        """
        ...

    @property
    def value(self) -> str:
        """
        Get the cookie's value.
        """
        ...

    @value.setter
    def set_value(self, name: str) -> None:
        """
        Set the cookie's value.
        """
        ...

    @property
    def http_only(self) -> Optional[bool]:
        """
        Whether the cookie is http only.
        """
        ...

    @http_only.setter
    def set_http_only(self, http_only: bool) -> None:
        """
        Set whether the cookie is http only.
        """
        ...

    @property
    def secure(self) -> Optional[bool]:
        """
        Whether the cookie is secure.
        """
        ...

    @secure.setter
    def set_secure(self, secure: bool) -> None:
        """
        Set whether the cookie is secure.
        """
        ...

    @property
    def same_site(self) -> Optional[SameSite]:
        """
        Get the cookie's same site.
        """
        ...

    @same_site.setter
    def set_same_site(self, same_site: SameSite) -> None:
        """
        Set the cookie's same site.
        """
        ...

    @property
    def partitioned(self) -> Optional[bool]:
        """
        Get the cookie's partitioned.
        """
        ...
    
    @partitioned.setter
    def set_partitioned(self, partitioned: bool) -> None:
        """
        Set the cookie's partitioned.
        """
        ...

    @property
    def max_age(self) -> Optional[float]:
        """
        Get the cookie's max age.
        """
        ...

    @max_age.setter
    def set_max_age(self, max_age: float) -> None:
        """
        Set the cookie's max age.
        """
        ...

    @property
    def path(self) -> Optional[str]:
        """
        Get the cookie's path.
        """
        ...

    @path.setter
    def set_path(self, path: str) -> None:
        """
        Set the cookie's path.
        """
        ...

    @property
    def domain(self) -> Optional[str]:
        """
        Get the cookie's domain.
        """
        ...

    @domain.setter
    def set_domain(self, domain: str) -> None:
        """
        Set the cookie's domain.
        """
        ...

    @property
    def expires(self) -> Optional[Expiration]:
        """
        Get the cookie's expiration.
        """
        ...

    @expires.setter
    def set_expires(self, expires: Expiration) -> None:
        """
        Set the cookie's expiration.
        """
        ...

    def make_removal(self) -> None:
        """
        Make the cookie removal.
        """
        ...

    def make_permanent(self) -> None:
        """
        Make the cookie permanent.
        """
        ...

    def encoded(self) -> str:
        """
        Get the encoded string.
        """
        ...


class Expiration:
    """
    Represents cookie expiration.
    """

    @staticmethod
    def session_expiration():
        """
        Create a session expiration.
        """
        ...

    @staticmethod
    def datetime_expiration(datetime: datetime):
        """
        Create a datetime expiration.
        """
        ...

    @property
    def is_session(self) -> bool:
        """
        Whether the expiration is session.
        """
        ...

    @property
    def is_datetime(self) -> bool:
        """
        Whether the expiration is datetime.
        """
        ...

    @property
    def datetime(self) -> Optional[datetime]:
        """
        Get the expiration datetime.
        """
        ...


class Request:
    """
    An HTTP request.
    """

    @property
    def version(self) -> str:
        """
        Get the request's HTTP version.
        """
        ...

    @version.setter
    def set_version(self, version: str) -> None:
        """
        Set the request's HTTP version.
        """
        ...

    @property
    def method(self) -> str:
        """
        Get the request's HTTP method.
        """
        ...

    @method.setter
    def set_method(self, method: str) -> None:
        """
        Set the request's HTTP method.
        """
        ...

    @property
    def uri(self) -> str:
        """
        Get the request's URI.
        """
        ...

    @uri.setter
    def set_uri(self, uri: str) -> None:
        """
        Set the request's URI.
        """
        ...

    @property
    def scheme(self) -> Optional[str]:
        """
        Get the request URI's scheme.
        """
        ...

    @property
    def host(self) -> Optional[str]:
        """
        Get the request URI's host.
        """
        ...

    @property
    def path(self) -> str:
        """
        Get the request URI's path.
        """
        ...

    @property
    def query(self) -> str:
        """
        Get the request URI's query string.
        """
        ...

    @property
    def content_type(self) -> Optional[str]:
        """
        Get the request's content type.
        """
        ...

    @property
    def headers(self) -> Headers:
        """
        Get the request's header map.
        """

    @headers.setter
    def set_headers(self, headers: Headers) -> None:
        """
        Set the request's header map.
        """

    @property
    def cookies(self) -> Cookies:
        """
        Get the cookies from the request.
        """
        ...

    @cookies.setter
    def set_cookies(self, cookies: Cookies) -> None:
        """
        Set the cookies to the request.
        """
        ...

    @property
    def body_object(self) -> Any:
        """
        Get the HTTP request's body object.
        """
        ...

    @property
    def set_body_object(self, new_value: Any) -> None:
        """
        Set the request's body object.
        """
        ...

    @property
    def teo(self) -> Any:
        """
        Get the ORM context.
        """
        ...

    @property
    def handler_match(self) -> HandlerMatch:
        """
        Get the handler match result.
        """
        ...

    @property
    def captures(self) -> dict[str, str]:
        """
        Get the path captures.
        """
        ...

    @property
    def local_values(self) -> LocalValues:
        """
        Get the request local values map.
        """
        ...

    @property
    def local_objects(self) -> LocalObjects:
        """
        Get the request local objects map.
        """
        ...


class LocalValues:
    """
    Represents request's local values.
    """

    def __setitem__(self, key: str, value: Any) -> None:
        """
        Insert a request local value.
        """
        ...

    def __getitem__(self, key: str) -> Optional[Any]:
        """
        Fetch a request local value by key.
        """
        ...

    def __hasitem__(self, key: str) -> bool:
        """
        Whether a request local value exists by key.
        """
        ...

    def __delitem__(self, key: str) -> None:
        """
        Remove a request local value by key.
        """
        ...

    def clear(self) -> None:
        """
        Clear request local values.
        """
        ...


class LocalObjects:
    """
    Represents request's local objects.
    """

    def __setitem__(self, key: str, value: Any) -> None:
        """
        Insert a request local object.
        """
        ...

    def __getitem__(self, key: str) -> Optional[Any]:
        """
        Fetch a request local object by key.
        """
        ...

    def __hasitem__(self, key: str) -> bool:
        """
        Whether a request local object exists by key.
        """
        ...

    def __delitem__(self, key: str) -> None:
        """
        Remove a request local object by key.
        """
        ...

    def clear(self) -> None:
        """
        Clear request local objects.
        """
        ...


class HandlerMatch:
    """
    The request handler match result.
    """

    @property
    def path(self) -> list[str]:
        """
        Get the matched handler's path.
        """
        ...

    @property
    def handler_name(self) -> str:
        """
        Get the handler's name.
        """
        ...

    @property
    def captures(self) -> dict[str, str]:
        """
        Get the URL path captures dict.
        """
        ...

    
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

    @property
    def upperbond(self) -> T:
        """
        Get the range's upperbond.
        """
        ...

    @property
    def lowerbond(self) -> T:
        """
        Get the range's lowerbond.
        """
        ...

    @property
    def is_closed(self) -> bool:
        """
        Whether this range is closed.
        """
        ...

    @property
    def is_open(self) -> bool:
        """
        Whether this range is open.
        """
        ...


class OptionVariant:
    """
    Option variant.
    """

    def __int__(self) -> int:
        ...

    def __and__(self, other: OptionVariant) -> OptionVariant:
        ...

    def __or__(self, other: OptionVariant) -> OptionVariant:
        ...

    def __xor__(self, other: OptionVariant) -> OptionVariant:
        ...

    def __invert__(self) -> OptionVariant:
        ...


class InterfaceEnumVariant:
    """
    Interface enum variant.
    """

    @property
    def value(self) -> str:
        """
        Get the variant's value.
        """
        ...

    def __getitem__(self, name: str) -> Optional[Any]:
        """
        Get the value of the argument with name.
        """
        ...


class Pipeline:
    pass


class PipelineCtx:
    """
    The context of pipeline.
    """

    @property
    def value(self) -> Any:
        """
        Get the context's current value.
        """
        ...

    @property
    def object(self) -> Any:
        """
        Get the context's current model object.
        """
        ...

    @property
    def path(self) -> list[str | int]:
        """
        Get the pipeline item's path.
        """
        ...

    @property
    def teo(self) -> Any:
        """
        Get Teo from this pipeline context.
        """
        ...

    @property
    def request(self) -> Optional[Request]:
        """
        Get the request from this pipeline context. This value may be none.
        """
        ...


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


class TeoException(Exception):

    error_message: str
    code: int
    errors: Optional[dict[str, str]]

    @property
    def message(self) -> str:
        pass

    def __init__(self, message: str, code: int = 500, errors: Optional[dict[str, str]] = None) -> None:
        pass

    def message_prefixed(self, prefix: str) -> TeoException:
        pass

    def path_prefixed(self, prefix: str) -> TeoException:
        pass

    def map_path(self, mapper: Callable[[str], str]) -> TeoException:
        pass

    @staticmethod
    def not_found(message: str = "not found") -> TeoException:
        pass

    @staticmethod
    def invalid_request(message: str = "value is invalid") -> TeoException:
        pass

    @staticmethod
    def internal_server_error(message: str = "internal server error") -> TeoException:
        pass

    @staticmethod
    def unauthorized(message: str = "unauthorized") -> TeoException:
        pass


class TestRequest:
    """
    Represents a test request.
    """
    ...

    def __init__(self, 
                 uri: str,
                 method: Optional[str] = None,
                 headers: Optional[dict[str, str]] = None,
                 body: Optional[bytes | str | Any] = None,
                 cookies: Optional[list[Cookie]] = None) -> None:
        """
        Create a new test request.
        """
        ...

    @property
    def method(self) -> str:
        """
        Get the request's method.
        """
        ...

    @method.setter
    def set_method(self, method: str) -> None:
        """
        Set the request's method.
        """
        ...

    @property
    def uri(self) -> str:
        """
        Get the request's URI.
        """
        ...

    @uri.setter
    def set_uri(self, uri: str) -> None:
        """
        Set the request's URI.
        """
        ...

    def insert_header(self, key: str, value: str) -> None:
        """
        Insert a header to the request.
        """
        ...

    def append_header(self, key: str, value: str) -> None:
        """
        Append a header to the request.
        """
        ...

    @property
    def body(self) -> Optional[bytes]:
        """
        Get the request's body.
        """
        ...

    @body.setter
    def set_body(self, body: bytes) -> None:
        """
        Set the request's body.
        """
        ...

    @property
    def cookies(self) -> Cookies:
        """
        Get the request's cookies.
        """
        ...


class TestResponse:
    """
    Represents a test response.
    """
    ...

    @property
    def status(self) -> int:
        """
        Get the response's status code.
        """
        ...

    @property
    def version(self) -> str:
        """
        Get the response's HTTP version.
        """
        ...

    def body(self) -> bytes:
        """
        Get the response's body.
        """
        ...

    def body_as_string(self) -> str:
        """
        Get the response's body as string.
        """
        ...

    def body_as_json(self) -> Any:
        """
        Get the response's body as JSON.
        """
        ...

    @property
    def headers(self) -> Headers:
        """
        Get the response's headers.
        """
        ...

    @property
    def cookies(self) -> Cookies:
        """
        Get the response's cookies.
        """
        ...


class TestServer:
    """
    Represents a test server.
    """
    ...

    def __init__(self, app: App) -> None:
        """
        Create a new test server with `app`.
        """
        ...

    async def setup(self):
        """
        Setup the test server for unit testing.
        """
        ...

    async def reset(self):
        """
        Reset the test server. This includes reset the database entries.
        """
        ...

    async def process(self, request: TestRequest) -> TestResponse:
        """
        Process a test request and return the response.
        """
        ...
