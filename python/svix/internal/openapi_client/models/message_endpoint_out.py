# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..models.message_status import MessageStatus
from ..types import UNSET, Unset

T = TypeVar("T", bound="MessageEndpointOut")


@attr.s(auto_attribs=True)
class MessageEndpointOut:
    """
    Attributes:
        created_at (datetime.datetime):
        description (str): An example endpoint name.
        id (str): The ep's ID Example: ep_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        status (MessageStatus): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        updated_at (datetime.datetime):
        url (str):  Example: https://example.com/webhook/.
        version (int):  Example: 1.
        channels (Union[Unset, None, List[str]]): List of message channels this endpoint listens to (omit for all).
            Example: ['project_123', 'group_2'].
        disabled (Union[Unset, bool]):
        filter_types (Union[Unset, None, List[str]]):  Example: ['user.signup', 'user.deleted'].
        next_attempt (Union[Unset, None, datetime.datetime]):
        rate_limit (Union[Unset, None, int]):
        uid (Union[Unset, None, str]): Optional unique identifier for the endpoint. Example: unique-ep-identifier.
    """

    created_at: datetime.datetime
    description: str
    id: str
    status: MessageStatus
    updated_at: datetime.datetime
    url: str
    version: int
    channels: Union[Unset, None, List[str]] = UNSET
    disabled: Union[Unset, bool] = False
    filter_types: Union[Unset, None, List[str]] = UNSET
    next_attempt: Union[Unset, None, datetime.datetime] = UNSET
    rate_limit: Union[Unset, None, int] = UNSET
    uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        description = self.description
        id = self.id
        status = self.status.value

        updated_at = self.updated_at.isoformat()

        url = self.url
        version = self.version
        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        disabled = self.disabled
        filter_types: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.filter_types, Unset):
            if self.filter_types is None:
                filter_types = None
            else:
                filter_types = self.filter_types

        next_attempt: Union[Unset, None, str] = UNSET
        if not isinstance(self.next_attempt, Unset):
            next_attempt = self.next_attempt.isoformat() if self.next_attempt else None

        rate_limit = self.rate_limit
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "description": description,
                "id": id,
                "status": status,
                "updatedAt": updated_at,
                "url": url,
                "version": version,
            }
        )
        if channels is not UNSET:
            field_dict["channels"] = channels
        if disabled is not UNSET:
            field_dict["disabled"] = disabled
        if filter_types is not UNSET:
            field_dict["filterTypes"] = filter_types
        if next_attempt is not UNSET:
            field_dict["nextAttempt"] = next_attempt
        if rate_limit is not UNSET:
            field_dict["rateLimit"] = rate_limit
        if uid is not UNSET:
            field_dict["uid"] = uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        description = d.pop("description")

        id = d.pop("id")

        status = MessageStatus(d.pop("status"))

        updated_at = isoparse(d.pop("updatedAt"))

        url = d.pop("url")

        version = d.pop("version")

        channels = cast(List[str], d.pop("channels", UNSET))

        disabled = d.pop("disabled", UNSET)

        filter_types = cast(List[str], d.pop("filterTypes", UNSET))

        _next_attempt = d.pop("nextAttempt", UNSET)
        next_attempt: Union[Unset, None, datetime.datetime]
        if _next_attempt is None:
            next_attempt = None
        elif isinstance(_next_attempt, Unset):
            next_attempt = UNSET
        else:
            next_attempt = isoparse(_next_attempt)

        rate_limit = d.pop("rateLimit", UNSET)

        uid = d.pop("uid", UNSET)

        message_endpoint_out = cls(
            created_at=created_at,
            description=description,
            id=id,
            status=status,
            updated_at=updated_at,
            url=url,
            version=version,
            channels=channels,
            disabled=disabled,
            filter_types=filter_types,
            next_attempt=next_attempt,
            rate_limit=rate_limit,
            uid=uid,
        )

        message_endpoint_out.additional_properties = d
        return message_endpoint_out

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
