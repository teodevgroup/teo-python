from os import path
from teo import App, Cookie, Response, Request
from tests.helpers.schema_path_args import schema_path_args


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def text_response(_: Request) -> Response:
        response = Response.string('foo', 'text/plain')
        response.cookies.append(Cookie('foo', 'bar'))
        return response
    app.main_namespace.define_handler('textResponse', text_response)
    def json_response() -> Response:
        response = Response.teon({'foo': 'bar'})
        response.cookies.append(Cookie('foo', 'bar'))
        return response
    app.main_namespace.define_handler('jsonResponse', json_response)
    def file_response(_: Request) -> Response:
        response = Response.file(path.join(path.dirname(__file__), 'response.txt'))
        response.cookies.append(Cookie('foo', 'bar'))
        return response
    app.main_namespace.define_handler('fileResponse', file_response)
    return app
