# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="HttpError")


@attr.s(auto_attribs=True)
class HttpError(Exception):
    """
    Attributes:
        code (str):
        detail (str):
    """

    code: str
    detail: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    status_code: int = 0

    @classmethod
    def init_exception(cls: Type[T], response: Dict[str, str], status_code: int) -> T:
        ret = cls.from_dict(response)
        ret.status_code = status_code
        return ret

    def to_dict(self) -> Dict[str, Any]:
        code = self.code
        detail = self.detail

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "code": code,
                "detail": detail,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        code = d.pop("code", "")

        detail = d.pop("detail", "")

        http_error = cls(
            code=code,
            detail=detail,
        )

        http_error.additional_properties = d
        return http_error

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
