# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.transformation_http_method import TransformationHttpMethod
from ..types import UNSET, Unset

T = TypeVar("T", bound="TransformationSimulateOut")


@attr.s(auto_attribs=True)
class TransformationSimulateOut:
    """
    Attributes:
        payload (str):
        url (str):
        method (Union[Unset, TransformationHttpMethod]):
    """

    payload: str
    url: str
    method: Union[Unset, TransformationHttpMethod] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        payload = self.payload
        url = self.url
        method: Union[Unset, str] = UNSET
        if not isinstance(self.method, Unset):
            method = self.method.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "payload": payload,
                "url": url,
            }
        )
        if method is not UNSET:
            field_dict["method"] = method

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        payload = d.pop("payload")

        url = d.pop("url")

        _method = d.pop("method", UNSET)
        method: Union[Unset, TransformationHttpMethod]
        if isinstance(_method, Unset):
            method = UNSET
        else:
            method = TransformationHttpMethod(_method)

        transformation_simulate_out = cls(
            payload=payload,
            url=url,
            method=method,
        )

        transformation_simulate_out.additional_properties = d
        return transformation_simulate_out

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
