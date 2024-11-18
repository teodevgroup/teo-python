from typing import Any, Callable, Optional
from decimal import Decimal
from datetime import datetime, date, timezone
from json import dumps
from re import compile, match


KeyPath = list[str | int]
Matcher = Callable[[KeyPath, Any], None]

def format_path(path: KeyPath) -> str:
    if len(path) == 0:
        return '(root)'
    if len(path) == 1:
        return str(path[0])
    result = str(path[0])
    for i in range(1, len(path)):
        current = path[i]
        if isinstance(current, int):
            result += f"[{current}]"
        else:
            result += f".{current}"
    return result

def display_value(value: Any) -> str:
    return dumps(value)

class JSONMatchError(Exception):
    def __init__(self, path: KeyPath, found: Any, reason: Optional[str] = None):
        super().__init__(f"{format_path(path)}: {reason or 'value is invalid'}\nFound value: {display_value(found)}")

def path_append(path: KeyPath, next: str | int) -> KeyPath:
    return path + [next]

def display_matcher(matcher: Any) -> str:
    if matcher == str:
        return 'string'
    elif matcher == int:
        return 'number'
    elif matcher == bool:
        return 'boolean'
    elif matcher == list:
        return 'array'
    else:
        if isinstance(matcher, list):
            return dumps(matcher)
        else:
            return dumps(matcher)

def match_json(value: Any, matcher: Any) -> Callable[[], None]:
    return lambda: match_json_value(value, matcher)

def match_json_value(value: Any, matcher: Any) -> None:
    match_json_value_pathed([], value, matcher)

def match_json_value_pathed(path: KeyPath, value: Any, matcher: Any) -> None:
    if callable(matcher):
        matcher(path, value)
    elif isinstance(value, str):
        if matcher == str:
            pass
        elif isinstance(matcher, str):
            if value != matcher:
                raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")
        else:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found '{display_value(value)}'")
    elif isinstance(value, bool):
        if matcher == bool:
            pass
        elif isinstance(matcher, bool):
            if value != matcher:
                raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")
        else:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found '{display_value(value)}'")
    elif isinstance(value, (int, float)):
        if matcher == bool:
            pass
        elif isinstance(matcher, (int, float)):
            if value != matcher:
                raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")
        else:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found '{display_value(value)}'")
    elif value is None:
        if matcher is not None:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")
    elif isinstance(value, list):
        if matcher == list:
            pass
        elif isinstance(matcher, list):
            match_array(path, value, matcher)
        else:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")
    elif isinstance(matcher, dict):
        if matcher == dict:
            pass
        elif isinstance(matcher, dict):
            match_object(path, value, matcher, False)
        else:
            raise JSONMatchError(path, value, f"expect {display_matcher(matcher)}, found {display_value(value)}")

def match_object(path: KeyPath, value: dict[str, Any], matcher: dict[str, Any], partial: bool) -> None:
    object_keys = value.keys()
    matcher_keys = matcher.keys()
    if not partial:
        for k in object_keys:
            if k not in matcher_keys:
                raise JSONMatchError(path, value, f"found extra key: {k}")
    for k in matcher_keys:
        if k not in object_keys:
            raise JSONMatchError(path, value, f"missing key: {k}")
    for k in matcher_keys:
        match_json_value_pathed(path_append(path, k), value[k], matcher[k])

def match_array(path: KeyPath, value: list[Any], matcher: list[Any]) -> None:
    if len(value) != len(matcher):
        raise JSONMatchError(path, value, "array of wrong length")
    for i in range(len(value)):
        match_json_value_pathed(path_append(path, i), value[i], matcher[i])

def partial(matcher: dict[str, Any]) -> Matcher:
    return lambda path, value: match_object(path, value, matcher, True)

def ignore(_path, _value) -> None:
    pass

def date_value(date_value: str | date) -> Matcher:
    def matcher(path: KeyPath, value: Any) -> None:
        if not isinstance(value, dict):
            raise JSONMatchError(path, value, "date value should be object")
        if len(value) != 1:
            raise JSONMatchError(path, value, "date object should have 1 key")
        if '$date' not in value:
            raise JSONMatchError(path, value, "date object should have 1 `$date` key")
        if isinstance(date_value, str):
            if value['$date'] != date_value:
                raise JSONMatchError(path, value, "value not equal")
        if isinstance(date_value, date):
            if value['$date'] != date_value.isoformat():
                raise JSONMatchError(path, value, "value not equal")
    return matcher

def datetime_value(datetime_value: str | datetime | int) -> Matcher:
    def matcher(path: KeyPath, value: Any) -> None:
        if not isinstance(value, dict):
            raise JSONMatchError(path, value, "date time value should be object")
        if len(value) != 1:
            raise JSONMatchError(path, value, "date time object should have 1 key")
        if '$datetime' not in value:
            raise JSONMatchError(path, value, "date time object should have 1 `$datetime` key")
        if isinstance(datetime_value, str):
            if value['$datetime'] != datetime_value:
                raise JSONMatchError(path, value, "value not equal")
        if isinstance(datetime_value, datetime):
            if value['$datetime'] != datetime_value.astimezone(timezone.utc).isoformat()[:23] + 'Z':
                raise JSONMatchError(path, value, "value not equal")
        if isinstance(datetime_value, int):
            if value['$datetime'] != datetime.fromtimestamp(datetime_value, timezone.utc).isoformat():
                raise JSONMatchError(path, value, "value not equal")
    return matcher


def decimal_value(decimal_value: Decimal | str | int) -> Matcher:
    def matcher(path: KeyPath, value: Any) -> None:
        if not isinstance(value, dict):
            raise JSONMatchError(path, value, "decimal value should be object")
        if len(value) != 1:
            raise JSONMatchError(path, value, "decimal object should have 1 key")
        if '$decimal' not in value:
            raise JSONMatchError(path, value, "decimal object should have 1 `$decimal` key")
        if value['$decimal'] != str(decimal_value):
            raise JSONMatchError(path, value, "value not equal")
    return matcher


def object_id_value(path: KeyPath, value: Any) -> None:
    if not isinstance(value, str):
        raise JSONMatchError(path, value, "object id value should be string")
    pattern = compile(r'^[0-9a-f]{24}$')
    if not match(pattern, value):
        raise JSONMatchError(path, value, "invalid object id value")

def ends_with(suffix: str) -> Matcher:
    def matcher(path: KeyPath, value: Any) -> None:
        if not value.endswith(suffix):
            raise JSONMatchError(path, value, f"value doesn't end with '{suffix}'")
    return matcher

def combine(*matchers: Matcher) -> Matcher:
    def matcher(path: KeyPath, value: Any) -> None:
        for m in matchers:
            m(path, value)
    return matcher

def one_matches(matcher: Any) -> Matcher:
    def match(path: KeyPath, value: Any) -> None:
        if not isinstance(value, list):
            raise JSONMatchError(path, value, "value is not array")
        for i in range(len(value)):
            try:
                match_json_value_pathed(path_append(path, i), value[i], matcher)
                return
            except JSONMatchError:
                pass
        raise JSONMatchError(path, value, "none of values matches matcher")
    return match

