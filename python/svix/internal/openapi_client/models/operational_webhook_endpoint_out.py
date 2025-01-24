# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.operational_webhook_endpoint_out_metadata import OperationalWebhookEndpointOutMetadata


T = TypeVar("T", bound="OperationalWebhookEndpointOut")


@attr.s(auto_attribs=True)
class OperationalWebhookEndpointOut:
    """
    Attributes:
        created_at (datetime.datetime):
        description (str): An example endpoint name.
        id (str): The ep's ID Example: ep_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        metadata (OperationalWebhookEndpointOutMetadata):
        updated_at (datetime.datetime):
        url (str):  Example: https://example.com/webhook/.
        disabled (Union[Unset, bool]):
        filter_types (Union[Unset, None, List[str]]):  Example: ['message.attempt.failing'].
        rate_limit (Union[Unset, None, int]):
        uid (Union[Unset, None, str]): Optional unique identifier for the endpoint. Example: unique-ep-identifier.
    """

    created_at: datetime.datetime
    description: str
    id: str
    metadata: "OperationalWebhookEndpointOutMetadata"
    updated_at: datetime.datetime
    url: str
    disabled: Union[Unset, bool] = False
    filter_types: Union[Unset, None, List[str]] = UNSET
    rate_limit: Union[Unset, None, int] = UNSET
    uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        description = self.description
        id = self.id
        metadata = self.metadata
        updated_at = self.updated_at.isoformat()

        url = self.url
        disabled = self.disabled
        filter_types: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.filter_types, Unset):
            if self.filter_types is None:
                filter_types = None
            else:
                filter_types = self.filter_types

        rate_limit = self.rate_limit
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "description": description,
                "id": id,
                "metadata": metadata,
                "updatedAt": updated_at,
                "url": url,
            }
        )
        if disabled is not UNSET:
            field_dict["disabled"] = disabled
        if filter_types is not UNSET:
            field_dict["filterTypes"] = filter_types
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

        metadata = d.pop("metadata")

        updated_at = isoparse(d.pop("updatedAt"))

        url = d.pop("url")

        disabled = d.pop("disabled", UNSET)

        filter_types = cast(List[str], d.pop("filterTypes", UNSET))

        rate_limit = d.pop("rateLimit", UNSET)

        uid = d.pop("uid", UNSET)

        operational_webhook_endpoint_out = cls(
            created_at=created_at,
            description=description,
            id=id,
            metadata=metadata,
            updated_at=updated_at,
            url=url,
            disabled=disabled,
            filter_types=filter_types,
            rate_limit=rate_limit,
            uid=uid,
        )

        operational_webhook_endpoint_out.additional_properties = d
        return operational_webhook_endpoint_out

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
