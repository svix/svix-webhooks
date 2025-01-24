# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="DashboardAccessOut")


@attr.s(auto_attribs=True)
class DashboardAccessOut:
    """
    Attributes:
        token (str):  Example: appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O.
        url (str):  Example: https://app.svix.com/login#key=eyJhcHBJZCI6ICJhcHBfMXRSdFl.
    """

    token: str
    url: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        token = self.token
        url = self.url

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "token": token,
                "url": url,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        token = d.pop("token")

        url = d.pop("url")

        dashboard_access_out = cls(
            token=token,
            url=url,
        )

        dashboard_access_out.additional_properties = d
        return dashboard_access_out

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
