# type: ignore
from __future__ import annotations
from typing import Any, Optional, Literal, Generic, TypeVar, NotRequired, TypedDict, cast, TYPE_CHECKING
from re import Pattern
from datetime import date, datetime
from decimal import Decimal
from teo import ObjectId, Enumerable, File, Range, OptionVariant
from teo.annotations import TeoAnnotationMark, ModelObjectAnnotationMark

from . import std


Sex = Literal["male", "female"]

SupportScalarFields = Literal["bool", "boolArray", "date", "dateArray", "dateTime", "dateTimeArray", "decimal", "decimalArray", "float32", "float32Array", "float64", "float64Array", "id", "int32", "int32Array", "int64", "int64Array", "sex", "sexesArray", "string", "stringArray"]

SupportSerializableScalarFields = Literal["bool", "boolArray", "date", "dateArray", "dateTime", "dateTimeArray", "decimal", "decimalArray", "float32", "float32Array", "float64", "float64Array", "id", "int32", "int32Array", "int64", "int64Array", "sex", "sexesArray", "string", "stringArray"]

SupportRelations = Literal[None]

SupportDirectRelations = Literal[None]

SupportIndirectRelations = Literal[None]



# **Support select**
#
# This synthesized interface doesn't have a description
class SupportSelect(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[bool]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[bool]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[bool]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[bool]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[bool]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[bool]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[bool]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[bool]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[bool]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[bool]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[bool]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[bool]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[bool]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[bool]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[bool]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[bool]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[bool]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[bool]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support include**
#
# This synthesized interface doesn't have a description
class SupportInclude(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Support where input**
#
# This synthesized interface doesn't have a description
class SupportWhereInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **And**
        #
        # This synthesized field doesn't have a description.
        "AND": "NotRequired[Optional[list[SupportWhereInput]]]",
        
        # **Not**
        #
        # This synthesized field doesn't have a description.
        "NOT": "NotRequired[Optional[SupportWhereInput]]",
        
        # **Or**
        #
        # This synthesized field doesn't have a description.
        "OR": "NotRequired[Optional[list[SupportWhereInput]]]",
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool | None | std.BoolNullableFilter]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool] | None | std.ArrayNullableFilter[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date | None | std.NullableFilter[date]]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date] | None | std.ArrayNullableFilter[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime | None | std.NullableFilter[datetime]]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime] | None | std.ArrayNullableFilter[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal | None | std.NullableFilter[Decimal]]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal] | None | std.ArrayNullableFilter[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float | None | std.NullableFilter[float]]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float] | None | std.ArrayNullableFilter[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float | None | std.NullableFilter[float]]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float] | None | std.ArrayNullableFilter[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int | std.Filter[int]]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int | None | std.NullableFilter[int]]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int] | None | std.ArrayNullableFilter[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int | None | std.NullableFilter[int]]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int] | None | std.ArrayNullableFilter[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex | None | std.EnumNullableFilter[Sex]]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex] | None | std.ArrayNullableFilter[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str | None | std.StringNullableFilter]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str] | None | std.ArrayNullableFilter[str]]]",
        
    },
)):
    pass

# **Support where unique input**
#
# This synthesized interface doesn't have a description
class SupportWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "int",
        
    },
)):
    pass

# **Support scalar where with aggregates input**
#
# This synthesized interface doesn't have a description
class SupportScalarWhereWithAggregatesInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **And**
        #
        # This synthesized field doesn't have a description.
        "AND": "NotRequired[Optional[list[SupportWhereInput]]]",
        
        # **Not**
        #
        # This synthesized field doesn't have a description.
        "NOT": "NotRequired[Optional[SupportWhereInput]]",
        
        # **Or**
        #
        # This synthesized field doesn't have a description.
        "OR": "NotRequired[Optional[list[SupportWhereInput]]]",
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool | None | std.BoolNullableWithAggregatesFilter]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool] | None | std.ArrayNullableWithAggregatesFilter[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date | None | std.NullableAggregatesFilter[date]]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date] | None | std.ArrayNullableWithAggregatesFilter[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime | None | std.NullableAggregatesFilter[datetime]]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime] | None | std.ArrayNullableWithAggregatesFilter[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal | None | std.DecimalNullableWithAggregatesFilter[Decimal]]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal] | None | std.ArrayNullableWithAggregatesFilter[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float | None | std.FloatNumberNullableWithAggregatesFilter[float]]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float] | None | std.ArrayNullableWithAggregatesFilter[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float | None | std.FloatNumberNullableWithAggregatesFilter[float]]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float] | None | std.ArrayNullableWithAggregatesFilter[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int | std.IntNumberWithAggregatesFilter[int]]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int | None | std.IntNumberNullableWithAggregatesFilter[int]]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int] | None | std.ArrayNullableWithAggregatesFilter[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int | None | std.IntNumberNullableWithAggregatesFilter[int]]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int] | None | std.ArrayNullableWithAggregatesFilter[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex | None | std.EnumNullableWithAggregatesFilter[Sex]]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex] | None | std.ArrayNullableWithAggregatesFilter[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str | None | std.StringNullableWithAggregatesFilter]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str] | None | std.ArrayNullableWithAggregatesFilter[str]]]",
        
    },
)):
    pass

# **Support relation filter**
#
# This synthesized interface doesn't have a description
class SupportRelationFilter(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Is**
        #
        # This synthesized field doesn't have a description.
        "is": "NotRequired[Optional[SupportWhereInput]]",
        
        # **Is Not**
        #
        # This synthesized field doesn't have a description.
        "isNot": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support list relation filter**
#
# This synthesized interface doesn't have a description
class SupportListRelationFilter(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Every**
        #
        # This synthesized field doesn't have a description.
        "every": "NotRequired[Optional[SupportWhereInput]]",
        
        # **None**
        #
        # This synthesized field doesn't have a description.
        "none": "NotRequired[Optional[SupportWhereInput]]",
        
        # **Some**
        #
        # This synthesized field doesn't have a description.
        "some": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support order by input**
#
# This synthesized interface doesn't have a description
class SupportOrderByInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[std.Sort]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[std.Sort]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[std.Sort]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[std.Sort]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[std.Sort]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[std.Sort]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[std.Sort]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[std.Sort]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[std.Sort]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[std.Sort]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[std.Sort]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[std.Sort]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[std.Sort]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[std.Sort]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[std.Sort]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[std.Sort]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[std.Sort]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[std.Sort]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[std.Sort]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[std.Sort]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[std.Sort]]",
        
    },
)):
    pass

# **Support count aggregate input type**
#
# This synthesized interface doesn't have a description
class SupportCountAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **All**
        #
        # This synthesized field doesn't have a description.
        "_all": "NotRequired[Optional[bool]]",
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[bool]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[bool]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[bool]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[bool]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[bool]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[bool]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[bool]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[bool]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[bool]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[bool]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[bool]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[bool]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[bool]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[bool]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[bool]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[bool]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[bool]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[bool]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support sum aggregate input type**
#
# This synthesized interface doesn't have a description
class SupportSumAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support avg aggregate input type**
#
# This synthesized interface doesn't have a description
class SupportAvgAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support min aggregate input type**
#
# This synthesized interface doesn't have a description
class SupportMinAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[bool]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[bool]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[bool]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[bool]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[bool]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[bool]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[bool]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[bool]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[bool]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[bool]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[bool]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[bool]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[bool]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[bool]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[bool]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[bool]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[bool]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[bool]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support max aggregate input type**
#
# This synthesized interface doesn't have a description
class SupportMaxAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[bool]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[bool]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[bool]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[bool]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[bool]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[bool]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[bool]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[bool]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[bool]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[bool]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[bool]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[bool]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[bool]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[bool]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[bool]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[bool]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[bool]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[bool]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Support create input**
#
# This synthesized interface doesn't have a description
class SupportCreateInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support update input**
#
# This synthesized interface doesn't have a description
class SupportUpdateInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support create nested one input**
#
# This synthesized interface doesn't have a description
class SupportCreateNestedOneInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[SupportConnectOrCreateInput]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[SupportCreateInput]]",
        
    },
)):
    pass

# **Support create nested many input**
#
# This synthesized interface doesn't have a description
class SupportCreateNestedManyInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[Enumerable[SupportWhereUniqueInput]]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[Enumerable[SupportConnectOrCreateInput]]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[Enumerable[SupportCreateInput]]]",
        
    },
)):
    pass

# **Support update nested one input**
#
# This synthesized interface doesn't have a description
class SupportUpdateNestedOneInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[SupportConnectOrCreateInput]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[SupportCreateInput]]",
        
        # **Delete**
        #
        # This synthesized field doesn't have a description.
        "delete": "NotRequired[Optional[bool]]",
        
        # **Disconnect**
        #
        # This synthesized field doesn't have a description.
        "disconnect": "NotRequired[Optional[bool]]",
        
        # **Set**
        #
        # This synthesized field doesn't have a description.
        "set": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "NotRequired[Optional[SupportUpdateInput]]",
        
        # **Upsert**
        #
        # This synthesized field doesn't have a description.
        "upsert": "NotRequired[Optional[SupportUpsertWithWhereUniqueInput]]",
        
    },
)):
    pass

# **Support update nested many input**
#
# This synthesized interface doesn't have a description
class SupportUpdateNestedManyInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[Enumerable[SupportWhereUniqueInput]]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[Enumerable[SupportConnectOrCreateInput]]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[Enumerable[SupportCreateInput]]]",
        
        # **Delete**
        #
        # This synthesized field doesn't have a description.
        "delete": "NotRequired[Optional[Enumerable[SupportWhereUniqueInput]]]",
        
        # **Delete Many**
        #
        # This synthesized field doesn't have a description.
        "deleteMany": "NotRequired[Optional[Enumerable[SupportWhereInput]]]",
        
        # **Disconnect**
        #
        # This synthesized field doesn't have a description.
        "disconnect": "NotRequired[Optional[Enumerable[SupportWhereUniqueInput]]]",
        
        # **Set**
        #
        # This synthesized field doesn't have a description.
        "set": "NotRequired[Optional[Enumerable[SupportWhereUniqueInput]]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "NotRequired[Optional[Enumerable[SupportUpdateWithWhereUniqueInput]]]",
        
        # **Update Many**
        #
        # This synthesized field doesn't have a description.
        "updateMany": "NotRequired[Optional[Enumerable[SupportUpdateManyWithWhereInput]]]",
        
        # **Upsert**
        #
        # This synthesized field doesn't have a description.
        "upsert": "NotRequired[Optional[Enumerable[SupportUpsertWithWhereUniqueInput]]]",
        
    },
)):
    pass

# **Support connect or create input**
#
# This synthesized interface doesn't have a description
class SupportConnectOrCreateInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "SupportCreateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support update with where unique input**
#
# This synthesized interface doesn't have a description
class SupportUpdateWithWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support upsert with where unique input**
#
# This synthesized interface doesn't have a description
class SupportUpsertWithWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "SupportCreateInput",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support update many with where input**
#
# This synthesized interface doesn't have a description
class SupportUpdateManyWithWhereInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereInput",
        
    },
)):
    pass

# **Support result**
#
# This synthesized interface doesn't have a description
class SupportResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "int",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support count aggregate result**
#
# This synthesized interface doesn't have a description
class SupportCountAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **All**
        #
        # This synthesized field doesn't have a description.
        "_all": "NotRequired[Optional[int]]",
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[int]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[int]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[int]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[int]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[int]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[int]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[int]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[int]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[int]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[int]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[int]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[int]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[int]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[int]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[int]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[int]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[int]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[int]]",
        
    },
)):
    pass

# **Support sum aggregate result**
#
# This synthesized interface doesn't have a description
class SupportSumAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
    },
)):
    pass

# **Support avg aggregate result**
#
# This synthesized interface doesn't have a description
class SupportAvgAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[float]]",
        
    },
)):
    pass

# **Support min aggregate result**
#
# This synthesized interface doesn't have a description
class SupportMinAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support max aggregate result**
#
# This synthesized interface doesn't have a description
class SupportMaxAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support aggregate result**
#
# This synthesized interface doesn't have a description
class SupportAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[SupportAvgAggregateResult]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[SupportCountAggregateResult]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[SupportMaxAggregateResult]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[SupportMinAggregateResult]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[SupportSumAggregateResult]]",
        
    },
)):
    pass

# **Support group by result**
#
# This synthesized interface doesn't have a description
class SupportGroupByResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[SupportAvgAggregateResult]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[SupportCountAggregateResult]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[SupportMaxAggregateResult]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[SupportMinAggregateResult]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[SupportSumAggregateResult]]",
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support args**
#
# This synthesized interface doesn't have a description
class SupportArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
    },
)):
    pass

# **Support find many args**
#
# This synthesized interface doesn't have a description
class SupportFindManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[SupportSerializableScalarFields]]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[SupportOrderByInput]]]",
        
        # **Page Number**
        #
        # This synthesized field doesn't have a description.
        "pageNumber": "NotRequired[Optional[int]]",
        
        # **Page Size**
        #
        # This synthesized field doesn't have a description.
        "pageSize": "NotRequired[Optional[int]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Skip**
        #
        # This synthesized field doesn't have a description.
        "skip": "NotRequired[Optional[int]]",
        
        # **Take**
        #
        # This synthesized field doesn't have a description.
        "take": "NotRequired[Optional[int]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support find first args**
#
# This synthesized interface doesn't have a description
class SupportFindFirstArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[SupportSerializableScalarFields]]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[SupportOrderByInput]]]",
        
        # **Page Number**
        #
        # This synthesized field doesn't have a description.
        "pageNumber": "NotRequired[Optional[int]]",
        
        # **Page Size**
        #
        # This synthesized field doesn't have a description.
        "pageSize": "NotRequired[Optional[int]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Skip**
        #
        # This synthesized field doesn't have a description.
        "skip": "NotRequired[Optional[int]]",
        
        # **Take**
        #
        # This synthesized field doesn't have a description.
        "take": "NotRequired[Optional[int]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support find unique args**
#
# This synthesized interface doesn't have a description
class SupportFindUniqueArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support create args**
#
# This synthesized interface doesn't have a description
class SupportCreateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "SupportCreateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
    },
)):
    pass

# **Support update args**
#
# This synthesized interface doesn't have a description
class SupportUpdateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support upsert args**
#
# This synthesized interface doesn't have a description
class SupportUpsertArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "SupportCreateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support copy args**
#
# This synthesized interface doesn't have a description
class SupportCopyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Copy**
        #
        # This synthesized field doesn't have a description.
        "copy": "SupportUpdateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support delete args**
#
# This synthesized interface doesn't have a description
class SupportDeleteArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereUniqueInput",
        
    },
)):
    pass

# **Support create many args**
#
# This synthesized interface doesn't have a description
class SupportCreateManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "Enumerable[SupportCreateInput]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
    },
)):
    pass

# **Support update many args**
#
# This synthesized interface doesn't have a description
class SupportUpdateManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "SupportUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereInput",
        
    },
)):
    pass

# **Support delete many args**
#
# This synthesized interface doesn't have a description
class SupportDeleteManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereInput",
        
    },
)):
    pass

# **Support copy many args**
#
# This synthesized interface doesn't have a description
class SupportCopyManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Copy**
        #
        # This synthesized field doesn't have a description.
        "copy": "SupportUpdateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "SupportWhereInput",
        
    },
)):
    pass

# **Support count args**
#
# This synthesized interface doesn't have a description
class SupportCountArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[SupportSerializableScalarFields]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[SupportOrderByInput]]]",
        
        # **Page Number**
        #
        # This synthesized field doesn't have a description.
        "pageNumber": "NotRequired[Optional[int]]",
        
        # **Page Size**
        #
        # This synthesized field doesn't have a description.
        "pageSize": "NotRequired[Optional[int]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportCountAggregateInputType]]",
        
        # **Skip**
        #
        # This synthesized field doesn't have a description.
        "skip": "NotRequired[Optional[int]]",
        
        # **Take**
        #
        # This synthesized field doesn't have a description.
        "take": "NotRequired[Optional[int]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support aggregate args**
#
# This synthesized interface doesn't have a description
class SupportAggregateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[SupportAvgAggregateInputType]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[SupportCountAggregateInputType]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[SupportMaxAggregateInputType]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[SupportMinAggregateInputType]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[SupportSumAggregateInputType]]",
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[SupportSerializableScalarFields]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[SupportOrderByInput]]]",
        
        # **Page Number**
        #
        # This synthesized field doesn't have a description.
        "pageNumber": "NotRequired[Optional[int]]",
        
        # **Page Size**
        #
        # This synthesized field doesn't have a description.
        "pageSize": "NotRequired[Optional[int]]",
        
        # **Skip**
        #
        # This synthesized field doesn't have a description.
        "skip": "NotRequired[Optional[int]]",
        
        # **Take**
        #
        # This synthesized field doesn't have a description.
        "take": "NotRequired[Optional[int]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support group by args**
#
# This synthesized interface doesn't have a description
class SupportGroupByArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[SupportAvgAggregateInputType]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[SupportCountAggregateInputType]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[SupportMaxAggregateInputType]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[SupportMinAggregateInputType]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[SupportSumAggregateInputType]]",
        
        # **By**
        #
        # This synthesized field doesn't have a description.
        "by": "Enumerable[SupportSerializableScalarFields]",
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[SupportWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[SupportSerializableScalarFields]]",
        
        # **Having**
        #
        # This synthesized field doesn't have a description.
        "having": "NotRequired[Optional[SupportScalarWhereWithAggregatesInput]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[SupportOrderByInput]]]",
        
        # **Page Number**
        #
        # This synthesized field doesn't have a description.
        "pageNumber": "NotRequired[Optional[int]]",
        
        # **Page Size**
        #
        # This synthesized field doesn't have a description.
        "pageSize": "NotRequired[Optional[int]]",
        
        # **Skip**
        #
        # This synthesized field doesn't have a description.
        "skip": "NotRequired[Optional[int]]",
        
        # **Take**
        #
        # This synthesized field doesn't have a description.
        "take": "NotRequired[Optional[int]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "NotRequired[Optional[SupportWhereInput]]",
        
    },
)):
    pass

# **Support scalar update input**
#
# This synthesized interface doesn't have a description
class SupportScalarUpdateInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Bool**
        #
        # This synthesized field doesn't have a description.
        "bool": "NotRequired[Optional[bool]]",
        
        # **Bool Array**
        #
        # This synthesized field doesn't have a description.
        "boolArray": "NotRequired[Optional[list[bool]]]",
        
        # **Date**
        #
        # This synthesized field doesn't have a description.
        "date": "NotRequired[Optional[date]]",
        
        # **Date Array**
        #
        # This synthesized field doesn't have a description.
        "dateArray": "NotRequired[Optional[list[date]]]",
        
        # **Date Time**
        #
        # This synthesized field doesn't have a description.
        "dateTime": "NotRequired[Optional[datetime]]",
        
        # **Date Time Array**
        #
        # This synthesized field doesn't have a description.
        "dateTimeArray": "NotRequired[Optional[list[datetime]]]",
        
        # **Decimal**
        #
        # This synthesized field doesn't have a description.
        "decimal": "NotRequired[Optional[Decimal]]",
        
        # **Decimal Array**
        #
        # This synthesized field doesn't have a description.
        "decimalArray": "NotRequired[Optional[list[Decimal]]]",
        
        # **Float32**
        #
        # This synthesized field doesn't have a description.
        "float32": "NotRequired[Optional[float]]",
        
        # **Float32 Array**
        #
        # This synthesized field doesn't have a description.
        "float32Array": "NotRequired[Optional[list[float]]]",
        
        # **Float64**
        #
        # This synthesized field doesn't have a description.
        "float64": "NotRequired[Optional[float]]",
        
        # **Float64 Array**
        #
        # This synthesized field doesn't have a description.
        "float64Array": "NotRequired[Optional[list[float]]]",
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
        # **Int32**
        #
        # This synthesized field doesn't have a description.
        "int32": "NotRequired[Optional[int]]",
        
        # **Int32 Array**
        #
        # This synthesized field doesn't have a description.
        "int32Array": "NotRequired[Optional[list[int]]]",
        
        # **Int64**
        #
        # This synthesized field doesn't have a description.
        "int64": "NotRequired[Optional[int]]",
        
        # **Int64 Array**
        #
        # This synthesized field doesn't have a description.
        "int64Array": "NotRequired[Optional[list[int]]]",
        
        # **Sex**
        #
        # This synthesized field doesn't have a description.
        "sex": "NotRequired[Optional[Sex]]",
        
        # **Sexes Array**
        #
        # This synthesized field doesn't have a description.
        "sexesArray": "NotRequired[Optional[list[Sex]]]",
        
        # **String**
        #
        # This synthesized field doesn't have a description.
        "string": "NotRequired[Optional[str]]",
        
        # **String Array**
        #
        # This synthesized field doesn't have a description.
        "stringArray": "NotRequired[Optional[list[str]]]",
        
    },
)):
    pass

# **Support sign in checker ids**
#
# This synthesized interface doesn't have a description
class SupportSignInCheckerIds(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Support sign in checker companions**
#
# This synthesized interface doesn't have a description
class SupportSignInCheckerCompanions(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Support sign in input**
#
# This synthesized interface doesn't have a description
class SupportSignInInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Credentials**
        #
        # This synthesized field doesn't have a description.
        "credentials": "SupportSignInArgs",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[SupportInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[SupportSelect]]",
        
    },
)):
    pass

# **Support sign in args**
#
# This synthesized interface doesn't have a description
class SupportSignInArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass



class SupportModel:
    async def find_many_objects(self, query: SupportFindManyArgs, /) -> list[Support]:
        return cast(Any, None)
    async def find_unique_object(self, query: SupportFindUniqueArgs, /) -> Optional[Support]:
        return cast(Any, None)
    async def find_first_object(self, query: SupportFindFirstArgs, /) -> Optional[Support]:
        return cast(Any, None)
    async def create_object(self, input: SupportCreateInput, /) -> Support:
        return cast(Any, None)
    async def count_objects(self, query: SupportCountArgs, /) -> int:
        return cast(Any, None)
    async def count_fields(self, query: SupportCountArgs, /) -> SupportCountAggregateResult:
        return cast(Any, None)
    async def aggregate(self, query: SupportAggregateArgs, /) -> SupportAggregateResult:
        return cast(Any, None)
    async def group_by(self, query: SupportGroupByArgs, /) -> list[SupportAggregateResult]:
        return cast(Any, None)
    
    async def sql(self, sql: str) -> list[Any]:
        return cast(Any, None)
    
class Support(ModelObjectAnnotationMark):
    def is_new(self) -> bool:
        return cast(Any, None)
    def is_modified(self) -> bool:
        return cast(Any, None)
    async def set(self, input: SupportUpdateInput, /) -> None:
        return cast(Any, None)
    async def update(self, input: SupportScalarUpdateInput, /) -> None:
        return cast(Any, None)
    async def save(self) -> None:
        return cast(Any, None)
    async def delete(self) -> None:
        return cast(Any, None)
    async def to_teon(self) -> SupportResult:
        return cast(Any, None)
    id: int
    int_32: Optional[int]
    int_64: Optional[int]
    float_32: Optional[float]
    float_64: Optional[float]
    bool: Optional[bool]
    string: Optional[str]
    date: Optional[date]
    date_time: Optional[datetime]
    decimal: Optional[Decimal]
    sex: Optional[Sex]
    int_32_array: Optional[list[int]]
    int_64_array: Optional[list[int]]
    float_32_array: Optional[list[float]]
    float_64_array: Optional[list[float]]
    bool_array: Optional[list[bool]]
    string_array: Optional[list[str]]
    date_array: Optional[list[date]]
    date_time_array: Optional[list[datetime]]
    decimal_array: Optional[list[Decimal]]
    sexes_array: Optional[list[Sex]]



class Teo(TeoAnnotationMark):

    
    async def transaction[T](self, teo: Teo, /) -> T:
        return cast(Any, None)
    
    support: SupportModel