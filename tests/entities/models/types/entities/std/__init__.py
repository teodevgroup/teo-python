from __future__ import annotations
from typing import Any, Optional, Literal, TypedDict, Generic, TypeVar, NotRequired, cast, TYPE_CHECKING
from re import Pattern
from datetime import date, datetime
from decimal import Decimal
from teo import ObjectId, Enumerable, File, Range, OptionVariant



from . import admin
from .admin import AdminNamespace

from . import bcrypt
from .bcrypt import BcryptNamespace

from . import identity
from .identity import IdentityNamespace



Sort = Literal["asc", "desc"]

StringMatchMode = Literal["default", "caseInsensitive"]



# **Empty**
#
# The empty interface
class Empty(TypedDict):

    pass



# **Data**
#
# This interface is common for action output
class Data[T](TypedDict):


    # **Data**
    #
    # This interface field doesn't have a description.
    data: T


# **Data and Meta**
#
# This interface is common for action output with meta information
class DataMeta[T, U](TypedDict):


    # **Data**
    #
    # This interface field doesn't have a description.
    data: T

    # **Meta**
    #
    # This interface field doesn't have a description.
    meta: U


# **Paging info**
#
# This interface doesn't have a description.
class PagingInfo(TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    count: int

    # **Number of pages**
    #
    # This interface field doesn't have a description.
    numberOfPages: NotRequired[Optional[int]]


# **Response error**
#
# This interface doesn't have a description.
class ResponseError(TypedDict):


    # **Type**
    #
    # This interface field doesn't have a description.
    type: str

    # **Message**
    #
    # This interface field doesn't have a description.
    message: str

    # **Fields**
    #
    # This interface field doesn't have a description.
    fields: dict[str, str] | None


# **Bool filter**
#
# This interface doesn't have a description.
class BoolFilter(TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[bool]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[bool | BoolFilter]]


# **Bool nullable filter**
#
# This interface doesn't have a description.
class BoolNullableFilter(TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[bool | None]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[bool | None | BoolNullableFilter]]


# **Filter**
#
# This interface doesn't have a description.
class Filter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[T]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[T]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[T]]]

    # **Lt**
    #
    # This interface field doesn't have a description.
    lt: NotRequired[Optional[T]]

    # **Lte**
    #
    # This interface field doesn't have a description.
    lte: NotRequired[Optional[T]]

    # **Gt**
    #
    # This interface field doesn't have a description.
    gt: NotRequired[Optional[T]]

    # **Gte**
    #
    # This interface field doesn't have a description.
    gte: NotRequired[Optional[T]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[T | Filter[T]]]


# **Nullable filter**
#
# This interface doesn't have a description.
class NullableFilter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[T | None]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[T | None]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[T | None]]]

    # **Lt**
    #
    # This interface field doesn't have a description.
    lt: NotRequired[Optional[T]]

    # **Lte**
    #
    # This interface field doesn't have a description.
    lte: NotRequired[Optional[T]]

    # **Gt**
    #
    # This interface field doesn't have a description.
    gt: NotRequired[Optional[T]]

    # **Gte**
    #
    # This interface field doesn't have a description.
    gte: NotRequired[Optional[T]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[T | None | NullableFilter[T]]]


# **String filter**
#
# This interface doesn't have a description.
class StringFilter(TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[str]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[str]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[str]]]

    # **Lt**
    #
    # This interface field doesn't have a description.
    lt: NotRequired[Optional[str]]

    # **Lte**
    #
    # This interface field doesn't have a description.
    lte: NotRequired[Optional[str]]

    # **Gt**
    #
    # This interface field doesn't have a description.
    gt: NotRequired[Optional[str]]

    # **Gte**
    #
    # This interface field doesn't have a description.
    gte: NotRequired[Optional[str]]

    # **Contains**
    #
    # This interface field doesn't have a description.
    contains: NotRequired[Optional[str]]

    # **Starts with**
    #
    # This interface field doesn't have a description.
    startsWith: NotRequired[Optional[str]]

    # **Ends with**
    #
    # This interface field doesn't have a description.
    endsWith: NotRequired[Optional[str]]

    # **Matches**
    #
    # This interface field doesn't have a description.
    matches: NotRequired[Optional[str]]

    # **Mode**
    #
    # This interface field doesn't have a description.
    mode: NotRequired[Optional[StringMatchMode]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[str | StringFilter]]


# **String nullable filter**
#
# This interface doesn't have a description.
class StringNullableFilter(TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[str | None]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[str | None]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[str | None]]]

    # **Lt**
    #
    # This interface field doesn't have a description.
    lt: NotRequired[Optional[str]]

    # **Lte**
    #
    # This interface field doesn't have a description.
    lte: NotRequired[Optional[str]]

    # **Gt**
    #
    # This interface field doesn't have a description.
    gt: NotRequired[Optional[str]]

    # **Gte**
    #
    # This interface field doesn't have a description.
    gte: NotRequired[Optional[str]]

    # **Contains**
    #
    # This interface field doesn't have a description.
    contains: NotRequired[Optional[str]]

    # **Starts with**
    #
    # This interface field doesn't have a description.
    startsWith: NotRequired[Optional[str]]

    # **Ends with**
    #
    # This interface field doesn't have a description.
    endsWith: NotRequired[Optional[str]]

    # **Matches**
    #
    # This interface field doesn't have a description.
    matches: NotRequired[Optional[str]]

    # **Mode**
    #
    # This interface field doesn't have a description.
    mode: NotRequired[Optional[StringMatchMode]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[str | None | StringNullableFilter]]


# **Enum filter**
#
# This interface doesn't have a description.
class EnumFilter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[T]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[T]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[T]]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[T | EnumFilter[T]]]


# **Enum nullable filter**
#
# This interface doesn't have a description.
class EnumNullableFilter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[T | None]]

    # **In**
    #
    # This interface field doesn't have a description.
    in_: NotRequired[Optional[list[T | None]]]

    # **Not in**
    #
    # This interface field doesn't have a description.
    notIn: NotRequired[Optional[list[T | None]]]

    # **Not**
    #
    # This interface field doesn't have a description.
    not_: NotRequired[Optional[T | None | EnumNullableFilter[T]]]


# **Array filter**
#
# This interface doesn't have a description.
class ArrayFilter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[list[T]]]

    # **Has**
    #
    # This interface field doesn't have a description.
    has: NotRequired[Optional[T]]

    # **Has some**
    #
    # This interface field doesn't have a description.
    hasSome: NotRequired[Optional[list[T]]]

    # **Has every**
    #
    # This interface field doesn't have a description.
    hasEvery: NotRequired[Optional[list[T]]]

    # **Is empty**
    #
    # This interface field doesn't have a description.
    isEmpty: NotRequired[Optional[bool]]

    # **Length**
    #
    # This interface field doesn't have a description.
    length: NotRequired[Optional[int]]


# **Array nullable filter**
#
# This interface doesn't have a description.
class ArrayNullableFilter[T](TypedDict):


    # **Equals**
    #
    # This interface field doesn't have a description.
    equals: NotRequired[Optional[list[T] | None]]

    # **Has**
    #
    # This interface field doesn't have a description.
    has: NotRequired[Optional[T]]

    # **Has some**
    #
    # This interface field doesn't have a description.
    hasSome: NotRequired[Optional[list[T]]]

    # **Has every**
    #
    # This interface field doesn't have a description.
    hasEvery: NotRequired[Optional[list[T]]]

    # **Is empty**
    #
    # This interface field doesn't have a description.
    isEmpty: NotRequired[Optional[bool]]

    # **Length**
    #
    # This interface field doesn't have a description.
    length: NotRequired[Optional[int]]


# **Bool with aggregates filter**
#
# This interface doesn't have a description.
class BoolWithAggregatesFilter(BoolFilter, TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[BoolFilter]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[BoolFilter]]


# **Bool nullable with aggregates filter**
#
# This interface doesn't have a description.
class BoolNullableWithAggregatesFilter(BoolNullableFilter, TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[BoolNullableFilter]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[BoolNullableFilter]]


# **Int number with aggregates filter**
#
# This interface doesn't have a description.
class IntNumberWithAggregatesFilter[T](Filter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[Filter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[Filter[T]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[Filter[float]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[Filter[int]]]


# **Int number nullable with aggregates filter**
#
# This interface doesn't have a description.
class IntNumberNullableWithAggregatesFilter[T](NullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[NullableFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[NullableFilter[T]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[NullableFilter[float]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[NullableFilter[int]]]


# **Float number with aggregates filter**
#
# This interface doesn't have a description.
class FloatNumberWithAggregatesFilter[T](Filter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[Filter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[Filter[T]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[Filter[float]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[Filter[float]]]


# **Float number nullable with aggregates filter**
#
# This interface doesn't have a description.
class FloatNumberNullableWithAggregatesFilter[T](NullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[NullableFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[NullableFilter[T]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[NullableFilter[float]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[NullableFilter[float]]]


# **Decimal with aggregates filter**
#
# This interface doesn't have a description.
class DecimalWithAggregatesFilter(Filter[Decimal], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[Filter[Decimal]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[Filter[Decimal]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[Filter[Decimal]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[Filter[Decimal]]]


# **Decimal nullable with aggregates filter**
#
# This interface doesn't have a description.
class DecimalNullableWithAggregatesFilter[T](NullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[NullableFilter[Decimal]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[NullableFilter[Decimal]]]

    # **Avg**
    #
    # This interface field doesn't have a description.
    _avg: NotRequired[Optional[NullableFilter[Decimal]]]

    # **Sum**
    #
    # This interface field doesn't have a description.
    _sum: NotRequired[Optional[NullableFilter[Decimal]]]


# **Aggregates filter**
#
# This interface doesn't have a description.
class AggregatesFilter[T](Filter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[Filter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[Filter[T]]]


# **Nullable aggregates filter**
#
# This interface doesn't have a description.
class NullableAggregatesFilter[T](NullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[NullableFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[NullableFilter[T]]]


# **String with aggregates filter**
#
# This interface doesn't have a description.
class StringWithAggregatesFilter(StringFilter, TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[StringFilter]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[StringFilter]]


# **String nullable with aggregates filter**
#
# This interface doesn't have a description.
class StringNullableWithAggregatesFilter(StringNullableFilter, TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[StringNullableFilter]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[StringNullableFilter]]


# **Enum with aggregates filter**
#
# This interface doesn't have a description.
class EnumWithAggregatesFilter[T](EnumFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[EnumFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[EnumFilter[T]]]


# **Enum nullable with aggregates filter**
#
# This interface doesn't have a description.
class EnumNullableWithAggregatesFilter[T](EnumNullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[EnumNullableFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[EnumNullableFilter[T]]]


# **Array with aggregates filter**
#
# This interface doesn't have a description.
class ArrayWithAggregatesFilter[T](ArrayFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[ArrayFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[ArrayFilter[T]]]


# **Array nullable with aggregates filter**
#
# This interface doesn't have a description.
class ArrayNullableWithAggregatesFilter[T](ArrayNullableFilter[T], TypedDict):


    # **Count**
    #
    # This interface field doesn't have a description.
    _count: NotRequired[Optional[int]]

    # **Min**
    #
    # This interface field doesn't have a description.
    _min: NotRequired[Optional[ArrayNullableFilter[T]]]

    # **Max**
    #
    # This interface field doesn't have a description.
    _max: NotRequired[Optional[ArrayNullableFilter[T]]]


# **Number atomic update operation input**
#
# This interface doesn't have a description.
class NumberAtomicUpdateOperationInput[T](TypedDict):


    # **Increment**
    #
    # This interface field doesn't have a description.
    increment: NotRequired[Optional[T]]

    # **Decrement**
    #
    # This interface field doesn't have a description.
    decrement: NotRequired[Optional[T]]

    # **Multiply**
    #
    # This interface field doesn't have a description.
    multiply: NotRequired[Optional[T]]

    # **Divide**
    #
    # This interface field doesn't have a description.
    divide: NotRequired[Optional[T]]


# **Array atomic update operation input**
#
# This interface doesn't have a description.
class ArrayAtomicUpdateOperationInput[T](TypedDict):


    # **Push**
    #
    # This interface field doesn't have a description.
    push: NotRequired[Optional[T]]







