from __future__ import annotations
from typing import Any, Optional, Literal, TypedDict, Generic, TypeVar, NotRequired, cast, TYPE_CHECKING
from re import Pattern
from datetime import date, datetime
from decimal import Decimal
from teo import ObjectId, Enumerable, File, Range, OptionVariant


if TYPE_CHECKING:
    from .. import entities






# **Token info**
#
# This interface doesn't have a description.
class TokenInfo(TypedDict):


    # **Token**
    #
    # This interface field doesn't have a description.
    token: str






class IdentityNamespace:

    
    pass
    