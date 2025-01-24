# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, cast

import attr

T = TypeVar("T", bound="ValidationError")


@attr.s(auto_attribs=True)
class ValidationError(Exception):
    """Validation errors have their own schema to provide context for invalid requests eg. mismatched types and out of
    bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY error.

        Attributes:
            loc (List[str]): The location as a [`Vec`] of [`String`]s -- often in the form `["body", "field_name"]`,
                `["query", "field_name"]`, etc. They may, however, be arbitrarily deep.
            msg (str): The message accompanying the validation error item.
            type (str): The type of error, often "type_error" or "value_error", but sometimes with more context like as
                "value_error.number.not_ge"
    """

    loc: List[str]
    msg: str
    type: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    status_code: int = 0

    @classmethod
    def init_exception(cls: Type[T], response: Dict[str, str], status_code: int) -> T:
        ret = cls.from_dict(response)
        ret.status_code = status_code
        return ret

    def to_dict(self) -> Dict[str, Any]:
        loc = self.loc

        msg = self.msg
        type = self.type

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "loc": loc,
                "msg": msg,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        loc = cast(List[str], d.pop("loc"))

        msg = d.pop("msg")

        type = d.pop("type")

        validation_error = cls(
            loc=loc,
            msg=msg,
            type=type,
        )

        validation_error.additional_properties = d
        return validation_error

    @property
    def additional_keys(self) -> List[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
