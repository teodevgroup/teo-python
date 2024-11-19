from teo import App, Response, Request
from tests.helpers.schema_path_args import schema_path_args


def load_app() -> App:
    app = App(schema_path_args(__file__, "schema.teo"))
    def inspect(request: Request) -> Response:
        request.local
        return Response.teon({

        })
    return app