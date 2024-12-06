from teo import App
from tests.helpers.schema_path_args import schema_path_args
from datetime import datetime, date, timedelta
from decimal import Decimal
from .entities import Container, Status


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
    @app.main.transform_pipeline_item_function("transformInt32Array")
    def transform_int_32_array(value: list[int]):
        return [v * 10 for v in value]
    @app.main.transform_pipeline_item_function("transformInt64Array")
    def transform_int_64_array(value: list[int]):
        return [v * 10 for v in value]
    @app.main.transform_pipeline_item_function("transformFloat32Array")
    def transform_float_32_array(value: list[float]):
        return [v * 10.0 for v in value]
    @app.main.transform_pipeline_item_function("transformFloat64Array")
    def transform_float_64_array(value: list[float]):
        return [v * 10.0 for v in value]
    @app.main.transform_pipeline_item_function("transformBoolArray")
    def transform_bool_array(value: list[bool]):
        return [not v for v in value]
    @app.main.transform_pipeline_item_function("transformStringArray")
    def transform_string_array(value: list[float]):
        return [f"*{v}*" for v in value]
    @app.main.transform_pipeline_item_function("transformDateArray")
    def transform_date_array(value: list[date]):
        return [v + timedelta(days=1) for v in value]
    @app.main.transform_pipeline_item_function("transformDateTimeArray")
    def transform_date_time_array(value: list[datetime]):
        return [v + timedelta(days=1) for v in value]
    @app.main.transform_pipeline_item_function("transformDecimalArray")
    def transform_decimal_array(value: list[Decimal]):
        return [v * 10 for v in value]
    @app.main.transform_pipeline_item_function("transformStatusArray")
    def transform_status_array(value: list[Status]) -> list[Status]:
        def mapper(value: Status) -> Status:
            match value:
                case 'open': return 'pending'
                case 'pending': return 'inProgress'
                case 'inProgress': return 'waitingForReview'
                case 'waitingForReview': return 'done'
                case 'done': return 'open'
                case _: raise Exception(f"unknown status {value}")
        return [mapper(v) for v in value]
    @app.main.transform_pipeline_item("alterInt32")
    def alter_int_32(to: int):
        return lambda: to
    @app.main.transform_pipeline_item("alterInt64")
    def alter_int_64(to: int):
        return lambda: to 
    @app.main.transform_pipeline_item("alterFloat32")
    def alter_float_32(to: float):
        return lambda: to 
    @app.main.transform_pipeline_item("alterFloat64")
    def alter_float_64(to: float):
        return lambda: to 
    @app.main.transform_pipeline_item("alterBool")
    def alter_bool(to: bool):
        return lambda: to 
    @app.main.transform_pipeline_item("alterString")
    def alter_string(to: str):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDate")
    def alter_date(to: date):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDateTime")
    def alter_date_time(to: datetime):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDecimal")
    def alter_decimal(to: Decimal):
        return lambda: to
    @app.main.transform_pipeline_item("alterStatus")
    def alter_status(to: Status):
        return lambda: to
    @app.main.transform_pipeline_item("alterInt32Array")
    def alter_int_32_array(to: list[int]):
        return lambda: to
    @app.main.transform_pipeline_item("alterInt64Array")
    def alter_int_64_array(to: list[int]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterFloat32Array")
    def alter_float_32_array(to: list[float]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterFloat64Array")
    def alter_float_64_array(to: list[float]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterBoolArray")
    def alter_bool_array(to: list[bool]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterStringArray")
    def alter_string_array(to: list[str]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDateArray")
    def alter_date_array(to: list[date]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDateTimeArray")
    def alter_date_time_array(to: list[datetime]):
        return lambda: to 
    @app.main.transform_pipeline_item("alterDecimalArray")
    def alter_decimal_array(to: list[Decimal]):
        return lambda: to
    @app.main.transform_pipeline_item("alterStatusArray")
    def alter_status_array(to: list[Status]):
        return lambda: to
    @app.main.validator_pipeline_item_function("validateInt32")
    def validate_int_32(value: int):
        return True
    @app.main.validator_pipeline_item_function("validateInt64")
    def validate_int_64(value: int):
        return None
    @app.main.validator_pipeline_item_function("validateFloat32")
    def validate_float_32(value: float):
        return True
    @app.main.validator_pipeline_item_function("validateFloat64")
    def validate_float_64(value: float):
        return True
    @app.main.validator_pipeline_item_function("validateBool")
    def validate_bool(value: float):
        return True
    @app.main.validator_pipeline_item_function("validateString")    
    def validate_string(value: str):
        if len(value) > 1:
            return None
        else:
            return "string is too short, expect length > 1"
    @app.main.validator_pipeline_item_function("validateDate")
    def validate_date(value: date):
        return True
    @app.main.validator_pipeline_item_function("validateDateTime")
    def validate_date_time(value: datetime):
        return True
    @app.main.validator_pipeline_item_function("validateDecimal")
    def validate_decimal(value: Decimal):
        return True
    @app.main.validator_pipeline_item_function("validateStatus")
    def validate_status(value: Status):
        return True
    @app.main.validator_pipeline_item_function("validateInt32Array")
    def validate_int_32_array(value: list[int]):
        return True
    @app.main.validator_pipeline_item_function("validateInt64Array")
    def validate_int_64_array(value: list[int]):
        return True
    @app.main.validator_pipeline_item_function("validateFloat32Array")
    def validate_float_32_array(value: list[float]):
        return True
    @app.main.validator_pipeline_item_function("validateFloat64Array")
    def validate_float_64_array(value: list[float]):
        return True
    @app.main.validator_pipeline_item_function("validateBoolArray")
    def validate_bool_array(value: list[float]):
        return True
    @app.main.validator_pipeline_item_function("validateStringArray")    
    def validate_string_array(value: list[str]):
        return True
    @app.main.validator_pipeline_item_function("validateDateArray")
    def validate_date_array(value: list[date]):
        return True
    @app.main.validator_pipeline_item_function("validateDateTimeArray")
    def validate_date_time_array(value: list[datetime]):
        return True
    @app.main.validator_pipeline_item_function("validateDecimalArray")
    def validate_decimal_array(value: list[Decimal]):
        return True
    @app.main.validator_pipeline_item_function("validateStatusArray")
    def validate_status_array(value: list[Status]):
        return True
    @app.main.callback_pipeline_item_function("int32Callback")
    def int_32_callback(value: int, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("int64Callback")
    def int_64_callback(value: int, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("float32Callback")
    def float_32_callback(value: float, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("float64Callback")
    def float_64_callback(value: float, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("boolCallback")
    def bool_callback(value: bool, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("stringCallback")
    def string_callback(value: str, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("dateCallback")
    def date_callback(value: date, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("dateTimeCallback")
    def date_time_callback(value: datetime, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("decimalCallback")
    def decimal_callback(value: Decimal, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("statusCallback")
    def status_callback(value: Status, container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("int32ArrayCallback")
    def int_32_array_callback(value: list[int], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("int64ArrayCallback")
    def int_64_array_callback(value: list[int], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("float32ArrayCallback")
    def float_32_array_callback(value: list[float], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("float64ArrayCallback")
    def float_64_array_callback(value: list[float], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("boolArrayCallback")
    def bool_array_callback(value: list[bool], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("stringArrayCallback")
    def string_array_callback(value: list[str], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("dateArrayCallback")
    def date_array_callback(value: list[date], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("dateTimeArrayCallback")
    def date_time_array_callback(value: list[datetime], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("decimalArrayCallback")
    def decimal_array_callback(value: list[Decimal], container: Container):
        container.message = f"{value}"
    @app.main.callback_pipeline_item_function("statusArrayCallback")
    def status_array_callback(value: list[Status], container: Container):
        container.message = f"{value}"
    @app.main.compare_pipeline_item_function("compareInt32")
    def compare_int_32(old: int, new: int):
        return old != new
    @app.main.compare_pipeline_item_function("compareInt64")
    def compare_int_64(old: int, new: int):
        return old != new
    @app.main.compare_pipeline_item_function("compareFloat32")
    def compare_float_32(old: float, new: float):
        return old != new
    @app.main.compare_pipeline_item_function("compareFloat64")
    def compare_float_64(old: float, new: float):
        return old != new
    @app.main.compare_pipeline_item_function("compareBool")
    def compare_bool(old: bool, new: bool):
        return old != new
    @app.main.compare_pipeline_item_function("compareString")
    def compare_string(old: str, new: str):
        return old != new
    @app.main.compare_pipeline_item_function("compareDate")
    def compare_date(old: date, new: date):
        return old != new
    @app.main.compare_pipeline_item_function("compareDateTime")
    def compare_date_time(old: datetime, new: datetime):
        return old != new
    @app.main.compare_pipeline_item_function("compareDecimal")
    def compare_decimal(old: Decimal, new: Decimal):
        return old != new
    @app.main.compare_pipeline_item_function("compareStatus")
    def compare_status(old: Status, new: Status):
        return old != new
    @app.main.compare_pipeline_item_function("compareInt32Array")
    def compare_int_32_array(old: list[int], new: list[int]):
        return old != new
    @app.main.compare_pipeline_item_function("compareInt64Array")
    def compare_int_64_array(old: list[int], new: list[int]):
        return old != new
    @app.main.compare_pipeline_item_function("compareFloat32Array")
    def compare_float_32_array(old: list[float], new: list[float]):
        return old != new
    @app.main.compare_pipeline_item_function("compareFloat64Array")
    def compare_float_64_array(old: list[float], new: list[float]):
        return old != new
    @app.main.compare_pipeline_item_function("compareBoolArray")
    def compare_bool_array(old: list[bool], new: list[bool]):
        return old != new
    @app.main.compare_pipeline_item_function("compareStringArray")
    def compare_string_array(old: list[str], new: list[str]):
        return old != new
    @app.main.compare_pipeline_item_function("compareDateArray")
    def compare_date_array(old: list[date], new: list[date]):
        return old != new
    @app.main.compare_pipeline_item_function("compareDateTimeArray")
    def compare_date_time_array(old: list[datetime], new: list[datetime]):
        return old != new
    @app.main.compare_pipeline_item_function("compareDecimalArray")
    def compare_decimal_array(old: list[Decimal], new: list[Decimal]):
        return old != new
    @app.main.compare_pipeline_item_function("compareStatusArray")
    def compare_status_array(old: list[Status], new: list[Status]):
        return old != new
    return app
