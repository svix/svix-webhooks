# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="ApplicationStats")


@attr.s(auto_attribs=True)
class ApplicationStats:
    """
    Attributes:
        app_id (str): The app's ID Example: app_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        message_destinations (int):
        app_uid (Union[Unset, None, str]): The app's UID Example: unique-app-identifier.
    """

    app_id: str
    message_destinations: int
    app_uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        app_id = self.app_id
        message_destinations = self.message_destinations
        app_uid = self.app_uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "appId": app_id,
                "messageDestinations": message_destinations,
            }
        )
        if app_uid is not UNSET:
            field_dict["appUid"] = app_uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        app_id = d.pop("appId")

        message_destinations = d.pop("messageDestinations")

        app_uid = d.pop("appUid", UNSET)

        application_stats = cls(
            app_id=app_id,
            message_destinations=message_destinations,
            app_uid=app_uid,
        )

        application_stats.additional_properties = d
        return application_stats

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
