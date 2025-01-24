# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.application_in import ApplicationIn
    from ..models.message_in_payload import MessageInPayload
    from ..models.message_in_transformations_params import MessageInTransformationsParams


T = TypeVar("T", bound="MessageIn")


@attr.s(auto_attribs=True)
class MessageIn:
    """
    Attributes:
        event_type (str): The event type's name Example: user.signup.
        payload (MessageInPayload): JSON payload to send as the request body of the webhook.

            We also support sending non-JSON payloads. Please contact us for more information. Example: {'email':
            'test@example.com', 'type': 'user.created', 'username': 'test_user'}.
        application (Union[Unset, ApplicationIn]):
        channels (Union[Unset, None, List[str]]): List of free-form identifiers that endpoints can filter by Example:
            ['project_123', 'group_2'].
        event_id (Union[Unset, None, str]): Optional unique identifier for the message Example: unique-msg-identifier.
        payload_retention_hours (Union[Unset, None, int]): Optional number of hours to retain the message payload. Note
            that this is mutually exclusive with `payloadRetentionPeriod`.
        payload_retention_period (Union[Unset, None, int]): Optional number of days to retain the message payload.
            Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`. Default: 90. Example: 90.
        tags (Union[Unset, None, List[str]]): List of free-form tags that can be filtered by when listing messages
            Example: ['my_tag', 'other'].
        transformations_params (Union[Unset, None, MessageInTransformationsParams]): Extra parameters to pass to
            Transformations (for future use)
    """

    event_type: str
    payload: "MessageInPayload"
    application: Union[Unset, "ApplicationIn"] = UNSET
    channels: Union[Unset, None, List[str]] = UNSET
    event_id: Union[Unset, None, str] = UNSET
    payload_retention_hours: Union[Unset, None, int] = UNSET
    payload_retention_period: Union[Unset, None, int] = 90
    tags: Union[Unset, None, List[str]] = UNSET
    transformations_params: Union[Unset, None, "MessageInTransformationsParams"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        event_type = self.event_type
        payload = self.payload
        application: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.application, Unset):
            application = self.application.to_dict()

        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        event_id = self.event_id
        payload_retention_hours = self.payload_retention_hours
        payload_retention_period = self.payload_retention_period
        tags: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.tags, Unset):
            if self.tags is None:
                tags = None
            else:
                tags = self.tags

        transformations_params = self.transformations_params

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "eventType": event_type,
                "payload": payload,
            }
        )
        if application is not UNSET:
            field_dict["application"] = application
        if channels is not UNSET:
            field_dict["channels"] = channels
        if event_id is not UNSET:
            field_dict["eventId"] = event_id
        if payload_retention_hours is not UNSET:
            field_dict["payloadRetentionHours"] = payload_retention_hours
        if payload_retention_period is not UNSET:
            field_dict["payloadRetentionPeriod"] = payload_retention_period
        if tags is not UNSET:
            field_dict["tags"] = tags
        if transformations_params is not UNSET:
            field_dict["transformationsParams"] = transformations_params

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.application_in import ApplicationIn

        d = src_dict.copy()
        event_type = d.pop("eventType")

        payload = d.pop("payload")

        _application = d.pop("application", UNSET)
        application: Union[Unset, ApplicationIn]
        if isinstance(_application, Unset):
            application = UNSET
        else:
            application = ApplicationIn.from_dict(_application)

        channels = cast(List[str], d.pop("channels", UNSET))

        event_id = d.pop("eventId", UNSET)

        payload_retention_hours = d.pop("payloadRetentionHours", UNSET)

        payload_retention_period = d.pop("payloadRetentionPeriod", UNSET)

        tags = cast(List[str], d.pop("tags", UNSET))

        transformations_params = d.pop("transformationsParams", UNSET)

        message_in = cls(
            event_type=event_type,
            payload=payload,
            application=application,
            channels=channels,
            event_id=event_id,
            payload_retention_hours=payload_retention_hours,
            payload_retention_period=payload_retention_period,
            tags=tags,
            transformations_params=transformations_params,
        )

        message_in.additional_properties = d
        return message_in

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
