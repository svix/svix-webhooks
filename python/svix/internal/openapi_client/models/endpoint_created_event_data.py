# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="EndpointCreatedEventData")


@attr.s(auto_attribs=True)
class EndpointCreatedEventData:
    """Sent when an endpoint is created, updated, or deleted

    Attributes:
        app_id (str): The app's ID Example: app_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        endpoint_id (str): The ep's ID Example: ep_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        app_uid (Union[Unset, None, str]): The app's UID Example: unique-app-identifier.
        endpoint_uid (Union[Unset, None, str]): The ep's UID Example: unique-ep-identifier.
    """

    app_id: str
    endpoint_id: str
    app_uid: Union[Unset, None, str] = UNSET
    endpoint_uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        app_id = self.app_id
        endpoint_id = self.endpoint_id
        app_uid = self.app_uid
        endpoint_uid = self.endpoint_uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "appId": app_id,
                "endpointId": endpoint_id,
            }
        )
        if app_uid is not UNSET:
            field_dict["appUid"] = app_uid
        if endpoint_uid is not UNSET:
            field_dict["endpointUid"] = endpoint_uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        app_id = d.pop("appId")

        endpoint_id = d.pop("endpointId")

        app_uid = d.pop("appUid", UNSET)

        endpoint_uid = d.pop("endpointUid", UNSET)

        endpoint_created_event_data = cls(
            app_id=app_id,
            endpoint_id=endpoint_id,
            app_uid=app_uid,
            endpoint_uid=endpoint_uid,
        )

        endpoint_created_event_data.additional_properties = d
        return endpoint_created_event_data

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
