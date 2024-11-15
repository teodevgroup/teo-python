from teo import App, Cookie, Response, Request
from tests.helpers.schema_path_args import schema_path_args


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def text_response(_request: Request) -> Response:
        response = Response.string('foo', 'text/plain')
        response.add_cookie(Cookie('foo', 'bar'))
        return response
    app.main_namespace().define_handler('text_response', text_response)
    def json_response(_request: Request) -> Response:
        response = Response.teon({'foo': 'bar'})
        response.add_cookie(Cookie('foo', 'bar'))
        return response
    app.main_namespace().define_handler('json_response', json_response)
    def file_response(_request: Request) -> Response:
        response = Response.file('response.txt')
        response.add_cookie(Cookie('foo', 'bar'))
        return response
    app.main_namespace().define_handler('file_response', file_response)
    return app
