# type: ignore
from __future__ import annotations
from typing import Any, Optional, Literal, Generic, TypeVar, NotRequired, TypedDict, cast, TYPE_CHECKING
from re import Pattern
from datetime import date, datetime
from decimal import Decimal
from teo import ObjectId, Enumerable, File, Range, OptionVariant
from teo.annotations import TeoAnnotationMark, ModelObjectAnnotationMark

from . import std


Status = Literal["open", "inProgress", "pending", "waitingForReview", "done"]

ContainerScalarFields = Literal["bool", "boolArray", "date", "dateArray", "dateTime", "dateTimeArray", "decimal", "decimalArray", "float32", "float32Array", "float64", "float64Array", "id", "int32", "int32Array", "int64", "int64Array", "message", "status", "statusArray", "string", "stringArray"]

ContainerSerializableScalarFields = Literal["bool", "boolArray", "date", "dateArray", "dateTime", "dateTimeArray", "decimal", "decimalArray", "float32", "float32Array", "float64", "float64Array", "id", "int32", "int32Array", "int64", "int64Array", "message", "status", "statusArray", "string", "stringArray"]

ContainerRelations = Literal[None]

ContainerDirectRelations = Literal[None]

ContainerIndirectRelations = Literal[None]



# **Container select**
#
# This synthesized interface doesn't have a description
class ContainerSelect(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[bool]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[bool]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[bool]]",
        
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

# **Container include**
#
# This synthesized interface doesn't have a description
class ContainerInclude(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Container where input**
#
# This synthesized interface doesn't have a description
class ContainerWhereInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **And**
        #
        # This synthesized field doesn't have a description.
        "AND": "NotRequired[Optional[list[ContainerWhereInput]]]",
        
        # **Not**
        #
        # This synthesized field doesn't have a description.
        "NOT": "NotRequired[Optional[ContainerWhereInput]]",
        
        # **Or**
        #
        # This synthesized field doesn't have a description.
        "OR": "NotRequired[Optional[list[ContainerWhereInput]]]",
        
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str | None | std.StringNullableFilter]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status | None | std.EnumNullableFilter[Status]]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status] | None | std.ArrayNullableFilter[Status]]]",
        
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

# **Container where unique input**
#
# This synthesized interface doesn't have a description
class ContainerWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "int",
        
    },
)):
    pass

# **Container scalar where with aggregates input**
#
# This synthesized interface doesn't have a description
class ContainerScalarWhereWithAggregatesInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **And**
        #
        # This synthesized field doesn't have a description.
        "AND": "NotRequired[Optional[list[ContainerWhereInput]]]",
        
        # **Not**
        #
        # This synthesized field doesn't have a description.
        "NOT": "NotRequired[Optional[ContainerWhereInput]]",
        
        # **Or**
        #
        # This synthesized field doesn't have a description.
        "OR": "NotRequired[Optional[list[ContainerWhereInput]]]",
        
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str | None | std.StringNullableWithAggregatesFilter]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status | None | std.EnumNullableWithAggregatesFilter[Status]]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status] | None | std.ArrayNullableWithAggregatesFilter[Status]]]",
        
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

# **Container relation filter**
#
# This synthesized interface doesn't have a description
class ContainerRelationFilter(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Is**
        #
        # This synthesized field doesn't have a description.
        "is": "NotRequired[Optional[ContainerWhereInput]]",
        
        # **Is Not**
        #
        # This synthesized field doesn't have a description.
        "isNot": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container list relation filter**
#
# This synthesized interface doesn't have a description
class ContainerListRelationFilter(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Every**
        #
        # This synthesized field doesn't have a description.
        "every": "NotRequired[Optional[ContainerWhereInput]]",
        
        # **None**
        #
        # This synthesized field doesn't have a description.
        "none": "NotRequired[Optional[ContainerWhereInput]]",
        
        # **Some**
        #
        # This synthesized field doesn't have a description.
        "some": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container order by input**
#
# This synthesized interface doesn't have a description
class ContainerOrderByInput(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[std.Sort]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[std.Sort]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[std.Sort]]",
        
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

# **Container count aggregate input type**
#
# This synthesized interface doesn't have a description
class ContainerCountAggregateInputType(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[bool]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[bool]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[bool]]",
        
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

# **Container sum aggregate input type**
#
# This synthesized interface doesn't have a description
class ContainerSumAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Container avg aggregate input type**
#
# This synthesized interface doesn't have a description
class ContainerAvgAggregateInputType(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[bool]]",
        
    },
)):
    pass

# **Container min aggregate input type**
#
# This synthesized interface doesn't have a description
class ContainerMinAggregateInputType(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[bool]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[bool]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[bool]]",
        
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

# **Container max aggregate input type**
#
# This synthesized interface doesn't have a description
class ContainerMaxAggregateInputType(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[bool]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[bool]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[bool]]",
        
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

# **Container create input**
#
# This synthesized interface doesn't have a description
class ContainerCreateInput(TypedDict(
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
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container update input**
#
# This synthesized interface doesn't have a description
class ContainerUpdateInput(TypedDict(
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
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container create nested one input**
#
# This synthesized interface doesn't have a description
class ContainerCreateNestedOneInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[ContainerConnectOrCreateInput]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[ContainerCreateInput]]",
        
    },
)):
    pass

# **Container create nested many input**
#
# This synthesized interface doesn't have a description
class ContainerCreateNestedManyInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[Enumerable[ContainerWhereUniqueInput]]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[Enumerable[ContainerConnectOrCreateInput]]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[Enumerable[ContainerCreateInput]]]",
        
    },
)):
    pass

# **Container update nested one input**
#
# This synthesized interface doesn't have a description
class ContainerUpdateNestedOneInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[ContainerConnectOrCreateInput]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[ContainerCreateInput]]",
        
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
        "set": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "NotRequired[Optional[ContainerUpdateInput]]",
        
        # **Upsert**
        #
        # This synthesized field doesn't have a description.
        "upsert": "NotRequired[Optional[ContainerUpsertWithWhereUniqueInput]]",
        
    },
)):
    pass

# **Container update nested many input**
#
# This synthesized interface doesn't have a description
class ContainerUpdateNestedManyInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Connect**
        #
        # This synthesized field doesn't have a description.
        "connect": "NotRequired[Optional[Enumerable[ContainerWhereUniqueInput]]]",
        
        # **Connect Or Create**
        #
        # This synthesized field doesn't have a description.
        "connectOrCreate": "NotRequired[Optional[Enumerable[ContainerConnectOrCreateInput]]]",
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "NotRequired[Optional[Enumerable[ContainerCreateInput]]]",
        
        # **Delete**
        #
        # This synthesized field doesn't have a description.
        "delete": "NotRequired[Optional[Enumerable[ContainerWhereUniqueInput]]]",
        
        # **Delete Many**
        #
        # This synthesized field doesn't have a description.
        "deleteMany": "NotRequired[Optional[Enumerable[ContainerWhereInput]]]",
        
        # **Disconnect**
        #
        # This synthesized field doesn't have a description.
        "disconnect": "NotRequired[Optional[Enumerable[ContainerWhereUniqueInput]]]",
        
        # **Set**
        #
        # This synthesized field doesn't have a description.
        "set": "NotRequired[Optional[Enumerable[ContainerWhereUniqueInput]]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "NotRequired[Optional[Enumerable[ContainerUpdateWithWhereUniqueInput]]]",
        
        # **Update Many**
        #
        # This synthesized field doesn't have a description.
        "updateMany": "NotRequired[Optional[Enumerable[ContainerUpdateManyWithWhereInput]]]",
        
        # **Upsert**
        #
        # This synthesized field doesn't have a description.
        "upsert": "NotRequired[Optional[Enumerable[ContainerUpsertWithWhereUniqueInput]]]",
        
    },
)):
    pass

# **Container connect or create input**
#
# This synthesized interface doesn't have a description
class ContainerConnectOrCreateInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "ContainerCreateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container update with where unique input**
#
# This synthesized interface doesn't have a description
class ContainerUpdateWithWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container upsert with where unique input**
#
# This synthesized interface doesn't have a description
class ContainerUpsertWithWhereUniqueInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "ContainerCreateInput",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container update many with where input**
#
# This synthesized interface doesn't have a description
class ContainerUpdateManyWithWhereInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereInput",
        
    },
)):
    pass

# **Container result**
#
# This synthesized interface doesn't have a description
class ContainerResult(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container count aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerCountAggregateResult(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[int]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[int]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[int]]",
        
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

# **Container sum aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerSumAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[int]]",
        
    },
)):
    pass

# **Container avg aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerAvgAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Id**
        #
        # This synthesized field doesn't have a description.
        "id": "NotRequired[Optional[float]]",
        
    },
)):
    pass

# **Container min aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerMinAggregateResult(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container max aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerMaxAggregateResult(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container aggregate result**
#
# This synthesized interface doesn't have a description
class ContainerAggregateResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[ContainerAvgAggregateResult]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[ContainerCountAggregateResult]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[ContainerMaxAggregateResult]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[ContainerMinAggregateResult]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[ContainerSumAggregateResult]]",
        
    },
)):
    pass

# **Container group by result**
#
# This synthesized interface doesn't have a description
class ContainerGroupByResult(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[ContainerAvgAggregateResult]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[ContainerCountAggregateResult]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[ContainerMaxAggregateResult]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[ContainerMinAggregateResult]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[ContainerSumAggregateResult]]",
        
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container args**
#
# This synthesized interface doesn't have a description
class ContainerArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
    },
)):
    pass

# **Container find many args**
#
# This synthesized interface doesn't have a description
class ContainerFindManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[ContainerSerializableScalarFields]]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[ContainerOrderByInput]]]",
        
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
        "select": "NotRequired[Optional[ContainerSelect]]",
        
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
        "where": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container find first args**
#
# This synthesized interface doesn't have a description
class ContainerFindFirstArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[ContainerSerializableScalarFields]]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[ContainerOrderByInput]]]",
        
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
        "select": "NotRequired[Optional[ContainerSelect]]",
        
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
        "where": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container find unique args**
#
# This synthesized interface doesn't have a description
class ContainerFindUniqueArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container create args**
#
# This synthesized interface doesn't have a description
class ContainerCreateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "ContainerCreateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
    },
)):
    pass

# **Container update args**
#
# This synthesized interface doesn't have a description
class ContainerUpdateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container upsert args**
#
# This synthesized interface doesn't have a description
class ContainerUpsertArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "ContainerCreateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container copy args**
#
# This synthesized interface doesn't have a description
class ContainerCopyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Copy**
        #
        # This synthesized field doesn't have a description.
        "copy": "ContainerUpdateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container delete args**
#
# This synthesized interface doesn't have a description
class ContainerDeleteArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereUniqueInput",
        
    },
)):
    pass

# **Container create many args**
#
# This synthesized interface doesn't have a description
class ContainerCreateManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Create**
        #
        # This synthesized field doesn't have a description.
        "create": "Enumerable[ContainerCreateInput]",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
    },
)):
    pass

# **Container update many args**
#
# This synthesized interface doesn't have a description
class ContainerUpdateManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Update**
        #
        # This synthesized field doesn't have a description.
        "update": "ContainerUpdateInput",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereInput",
        
    },
)):
    pass

# **Container delete many args**
#
# This synthesized interface doesn't have a description
class ContainerDeleteManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereInput",
        
    },
)):
    pass

# **Container copy many args**
#
# This synthesized interface doesn't have a description
class ContainerCopyManyArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Copy**
        #
        # This synthesized field doesn't have a description.
        "copy": "ContainerUpdateInput",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
        # **Where**
        #
        # This synthesized field doesn't have a description.
        "where": "ContainerWhereInput",
        
    },
)):
    pass

# **Container count args**
#
# This synthesized interface doesn't have a description
class ContainerCountArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[ContainerSerializableScalarFields]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[ContainerOrderByInput]]]",
        
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
        "select": "NotRequired[Optional[ContainerCountAggregateInputType]]",
        
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
        "where": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container aggregate args**
#
# This synthesized interface doesn't have a description
class ContainerAggregateArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[ContainerAvgAggregateInputType]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[ContainerCountAggregateInputType]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[ContainerMaxAggregateInputType]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[ContainerMinAggregateInputType]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[ContainerSumAggregateInputType]]",
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[ContainerSerializableScalarFields]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[ContainerOrderByInput]]]",
        
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
        "where": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container group by args**
#
# This synthesized interface doesn't have a description
class ContainerGroupByArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Avg**
        #
        # This synthesized field doesn't have a description.
        "_avg": "NotRequired[Optional[ContainerAvgAggregateInputType]]",
        
        # **Count**
        #
        # This synthesized field doesn't have a description.
        "_count": "NotRequired[Optional[ContainerCountAggregateInputType]]",
        
        # **Max**
        #
        # This synthesized field doesn't have a description.
        "_max": "NotRequired[Optional[ContainerMaxAggregateInputType]]",
        
        # **Min**
        #
        # This synthesized field doesn't have a description.
        "_min": "NotRequired[Optional[ContainerMinAggregateInputType]]",
        
        # **Sum**
        #
        # This synthesized field doesn't have a description.
        "_sum": "NotRequired[Optional[ContainerSumAggregateInputType]]",
        
        # **By**
        #
        # This synthesized field doesn't have a description.
        "by": "Enumerable[ContainerSerializableScalarFields]",
        
        # **Cursor**
        #
        # This synthesized field doesn't have a description.
        "cursor": "NotRequired[Optional[ContainerWhereUniqueInput]]",
        
        # **Distinct**
        #
        # This synthesized field doesn't have a description.
        "distinct": "NotRequired[Optional[ContainerSerializableScalarFields]]",
        
        # **Having**
        #
        # This synthesized field doesn't have a description.
        "having": "NotRequired[Optional[ContainerScalarWhereWithAggregatesInput]]",
        
        # **Order By**
        #
        # This synthesized field doesn't have a description.
        "orderBy": "NotRequired[Optional[Enumerable[ContainerOrderByInput]]]",
        
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
        "where": "NotRequired[Optional[ContainerWhereInput]]",
        
    },
)):
    pass

# **Container scalar update input**
#
# This synthesized interface doesn't have a description
class ContainerScalarUpdateInput(TypedDict(
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
        
        # **Message**
        #
        # This synthesized field doesn't have a description.
        "message": "NotRequired[Optional[str]]",
        
        # **Status**
        #
        # This synthesized field doesn't have a description.
        "status": "NotRequired[Optional[Status]]",
        
        # **Status Array**
        #
        # This synthesized field doesn't have a description.
        "statusArray": "NotRequired[Optional[list[Status]]]",
        
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

# **Container sign in checker ids**
#
# This synthesized interface doesn't have a description
class ContainerSignInCheckerIds(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Container sign in checker companions**
#
# This synthesized interface doesn't have a description
class ContainerSignInCheckerCompanions(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass

# **Container sign in input**
#
# This synthesized interface doesn't have a description
class ContainerSignInInput(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
        # **Credentials**
        #
        # This synthesized field doesn't have a description.
        "credentials": "ContainerSignInArgs",
        
        # **Include**
        #
        # This synthesized field doesn't have a description.
        "include": "NotRequired[Optional[ContainerInclude]]",
        
        # **Select**
        #
        # This synthesized field doesn't have a description.
        "select": "NotRequired[Optional[ContainerSelect]]",
        
    },
)):
    pass

# **Container sign in args**
#
# This synthesized interface doesn't have a description
class ContainerSignInArgs(TypedDict(
    "RequestBodyObjectAnnotationMark",
    {
        
    },
)):
    pass



class ContainerModel:
    async def find_many_objects(self, query: ContainerFindManyArgs, /) -> list[Container]:
        return cast(Any, None)
    async def find_unique_object(self, query: ContainerFindUniqueArgs, /) -> Optional[Container]:
        return cast(Any, None)
    async def find_first_object(self, query: ContainerFindFirstArgs, /) -> Optional[Container]:
        return cast(Any, None)
    async def create_object(self, input: ContainerCreateInput, /) -> Container:
        return cast(Any, None)
    async def count_objects(self, query: ContainerCountArgs, /) -> int:
        return cast(Any, None)
    async def count_fields(self, query: ContainerCountArgs, /) -> ContainerCountAggregateResult:
        return cast(Any, None)
    async def aggregate(self, query: ContainerAggregateArgs, /) -> ContainerAggregateResult:
        return cast(Any, None)
    async def group_by(self, query: ContainerGroupByArgs, /) -> list[ContainerAggregateResult]:
        return cast(Any, None)
    
    async def sql(self, sql: str) -> list[Any]:
        return cast(Any, None)
    
class Container(ModelObjectAnnotationMark):
    def is_new(self) -> bool:
        return cast(Any, None)
    def is_modified(self) -> bool:
        return cast(Any, None)
    async def set(self, input: ContainerUpdateInput, /) -> None:
        return cast(Any, None)
    async def update(self, input: ContainerScalarUpdateInput, /) -> None:
        return cast(Any, None)
    async def save(self) -> None:
        return cast(Any, None)
    async def delete(self) -> None:
        return cast(Any, None)
    async def to_teon(self) -> ContainerResult:
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
    status: Optional[Status]
    int_32_array: Optional[list[int]]
    int_64_array: Optional[list[int]]
    float_32_array: Optional[list[float]]
    float_64_array: Optional[list[float]]
    bool_array: Optional[list[bool]]
    string_array: Optional[list[str]]
    date_array: Optional[list[date]]
    date_time_array: Optional[list[datetime]]
    decimal_array: Optional[list[Decimal]]
    status_array: Optional[list[Status]]
    message: Optional[str]



class Teo(TeoAnnotationMark):

    
    async def transaction[T](self, teo: Teo, /) -> T:
        return cast(Any, None)
    
    container: ContainerModel