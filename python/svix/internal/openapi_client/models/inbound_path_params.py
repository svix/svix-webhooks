# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="InboundPathParams")


@attr.s(auto_attribs=True)
class InboundPathParams:
    """
    Attributes:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        inbound_token (str):
    """

    app_id: str
    inbound_token: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        app_id = self.app_id
        inbound_token = self.inbound_token

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "app_id": app_id,
                "inbound_token": inbound_token,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        app_id = d.pop("app_id")

        inbound_token = d.pop("inbound_token")

        inbound_path_params = cls(
            app_id=app_id,
            inbound_token=inbound_token,
        )

        inbound_path_params.additional_properties = d
        return inbound_path_params

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
