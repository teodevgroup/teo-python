from teo import App, Request, Response, HandlerGroup
from tests.helpers.schema_path_args import schema_path_args
from datetime import datetime, date, timedelta
from decimal import Decimal
from .entities import Teo, Container, Status


def load_app():
    app = App(schema_path_args(__file__, "schema.teo"))
    @app.main.transform_pipeline_item_function("transformInt32")
    def transform_int_32(value: int):
        return value * 10
    @app.main.transform_pipeline_item_function("transformInt64")
    def transform_int_64(value: int):
        return value * 10
    @app.main.transform_pipeline_item_function("transformFloat32")
    def transform_float_32(value: float):
        return value * 10.0
    @app.main.transform_pipeline_item_function("transformFloat64")
    def transform_float_64(value: float):
        return value * 10.0
    @app.main.transform_pipeline_item_function("transformBool")
    def transform_bool(value: bool):
        return not value
    @app.main.transform_pipeline_item_function("transformString")
    def transform_string(value: float):
        return f"*{value}*"
    @app.main.transform_pipeline_item_function("transformDate")
    def transform_date(value: date):
        return value + timedelta(days=1)
    @app.main.transform_pipeline_item_function("transformDateTime")
    def transform_date_time(value: datetime):
        return value + timedelta(days=1)
    @app.main.transform_pipeline_item_function("transformDecimal")
    def transform_decimal(value: Decimal):
        return value * 10
    @app.main.transform_pipeline_item_function("transformStatus")
    def transform_status(value: Status) -> Status:
        match value:
            case 'open': return 'pending'
            case 'pending': return 'inProgress'
            case 'inProgress': return 'waitingForReview'
            case 'waitingForReview': return 'done'
            case 'done': return 'open'
            case _: raise Exception(f"unknown status {value}")
    
    return app
