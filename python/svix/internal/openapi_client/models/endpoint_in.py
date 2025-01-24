# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.endpoint_in_metadata import EndpointInMetadata


T = TypeVar("T", bound="EndpointIn")


@attr.s(auto_attribs=True)
class EndpointIn:
    """
    Attributes:
        url (str):  Example: https://example.com/webhook/.
        channels (Union[Unset, None, List[str]]): List of message channels this endpoint listens to (omit for all).
            Example: ['project_123', 'group_2'].
        description (Union[Unset, str]):  Default: ''. Example: An example endpoint name.
        disabled (Union[Unset, bool]):
        filter_types (Union[Unset, None, List[str]]):  Example: ['user.signup', 'user.deleted'].
        metadata (Union[Unset, EndpointInMetadata]):
        rate_limit (Union[Unset, None, int]):
        secret (Union[Unset, None, str]): The endpoint's verification secret.

            Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
            It is recommended to not set this and let the server generate the secret. Example:
            whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD.
        uid (Union[Unset, None, str]): Optional unique identifier for the endpoint. Example: unique-ep-identifier.
        version (Union[Unset, None, int]):  Default: 1. Example: 1.
    """

    url: str
    channels: Union[Unset, None, List[str]] = UNSET
    description: Union[Unset, str] = ""
    disabled: Union[Unset, bool] = False
    filter_types: Union[Unset, None, List[str]] = UNSET
    metadata: Union[Unset, "EndpointInMetadata"] = UNSET
    rate_limit: Union[Unset, None, int] = UNSET
    secret: Union[Unset, None, str] = UNSET
    uid: Union[Unset, None, str] = UNSET
    version: Union[Unset, None, int] = 1
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        url = self.url
        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        description = self.description
        disabled = self.disabled
        filter_types: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.filter_types, Unset):
            if self.filter_types is None:
                filter_types = None
            else:
                filter_types = self.filter_types

        metadata = self.metadata
        rate_limit = self.rate_limit
        secret = self.secret
        uid = self.uid
        version = self.version

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "url": url,
            }
        )
        if channels is not UNSET:
            field_dict["channels"] = channels
        if description is not UNSET:
            field_dict["description"] = description
        if disabled is not UNSET:
            field_dict["disabled"] = disabled
        if filter_types is not UNSET:
            field_dict["filterTypes"] = filter_types
        if metadata is not UNSET:
            field_dict["metadata"] = metadata
        if rate_limit is not UNSET:
            field_dict["rateLimit"] = rate_limit
        if secret is not UNSET:
            field_dict["secret"] = secret
        if uid is not UNSET:
            field_dict["uid"] = uid
        if version is not UNSET:
            field_dict["version"] = version

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        url = d.pop("url")

        channels = cast(List[str], d.pop("channels", UNSET))

        description = d.pop("description", UNSET)

        disabled = d.pop("disabled", UNSET)

        filter_types = cast(List[str], d.pop("filterTypes", UNSET))

        metadata = d.pop("metadata", UNSET)

        rate_limit = d.pop("rateLimit", UNSET)

        secret = d.pop("secret", UNSET)

        uid = d.pop("uid", UNSET)

        version = d.pop("version", UNSET)

        endpoint_in = cls(
            url=url,
            channels=channels,
            description=description,
            disabled=disabled,
            filter_types=filter_types,
            metadata=metadata,
            rate_limit=rate_limit,
            secret=secret,
            uid=uid,
            version=version,
        )

        endpoint_in.additional_properties = d
        return endpoint_in

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
