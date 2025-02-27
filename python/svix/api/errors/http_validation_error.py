# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

from .validation_error import ValidationError

T = TypeVar("T", bound="HTTPValidationError")


@attr.s(auto_attribs=True)
class HTTPValidationError(Exception):
    """
    Attributes:
        detail (List['ValidationError']):
    """

    detail: List[ValidationError]
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    status_code: int = 0

    @classmethod
    def init_exception(cls: Type[T], response: Dict[str, str], status_code: int) -> T:
        ret = cls.from_dict(response)
        ret.status_code = status_code
        return ret

    def to_dict(self) -> Dict[str, Any]:
        detail = []
        for detail_item_data in self.detail:
            detail_item = detail_item_data.to_dict()

            detail.append(detail_item)

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "detail": detail,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        detail = []
        _detail = d.pop("detail", "")
        for detail_item_data in _detail:
            detail_item = ValidationError.from_dict(detail_item_data)

            detail.append(detail_item)

        http_validation_error = cls(
            detail=detail,
        )

        http_validation_error.additional_properties = d
        return http_validation_error

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
