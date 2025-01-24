# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar

import attr
from dateutil.parser import isoparse

T = TypeVar("T", bound="MessageAttemptFailedData")


@attr.s(auto_attribs=True)
class MessageAttemptFailedData:
    """
    Attributes:
        id (str): The attempt's ID Example: atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        response_status_code (int):
        timestamp (datetime.datetime):
    """

    id: str
    response_status_code: int
    timestamp: datetime.datetime
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        id = self.id
        response_status_code = self.response_status_code
        timestamp = self.timestamp.isoformat()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "id": id,
                "responseStatusCode": response_status_code,
                "timestamp": timestamp,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        id = d.pop("id")

        response_status_code = d.pop("responseStatusCode")

        timestamp = isoparse(d.pop("timestamp"))

        message_attempt_failed_data = cls(
            id=id,
            response_status_code=response_status_code,
            timestamp=timestamp,
        )

        message_attempt_failed_data.additional_properties = d
        return message_attempt_failed_data

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
