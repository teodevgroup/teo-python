from teo import App, Response, Request
from tests.helpers.schema_path_args import schema_path_args


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def inspect(request: Request) -> Response:
        content_type = request.header_value('content-type')
        return Response.teon({
            'path': request.path(),
            'query': request.query(),
            'contentTypeFromHeader': content_type,
            'contentType': request.content_type(),
            'method': request.method(),
        })
    app.main_namespace().define_handler('inspect', inspect)
    def echo(request: Request) -> Response:
        captures = request.captures()
        echo = captures['data']
        return Response.string(echo, 'text/plain')
    app.main_namespace().define_handler('echo', echo)
    def echo_more(request: Request) -> Response:
        captures = request.captures()
        echo = captures['data']
        return Response.string(echo, 'text/plain')
    app.main_namespace().define_handler('echoMore', echo_more)
    def echo_json_body(request: Request) -> Response:
        return Response.teon(request.body_object())
    app.main_namespace().define_handler('echoJsonBody', echo_json_body)
    def echo_form_body(request: Request) -> Response:
        filepath = request.body_object()['avatar'].filepath
        return Response.teon({
            'name': request.body_object()['name'],
            'avatar': filepath
        })
    app.main_namespace().define_handler('echoFormBody', echo_form_body)
    def echo_cookie(request: Request) -> Response:
        return Response.teon({
            'cookies': list(map(lambda cookie: {
                'name': cookie.name(),
                'value': cookie.value(),
            }, request.cookies()))
        })
    app.main_namespace().define_handler('echoCookie', echo_cookie)
    return app
