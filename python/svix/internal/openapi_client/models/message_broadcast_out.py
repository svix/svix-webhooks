# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

from ..models.background_task_status import BackgroundTaskStatus
from ..models.background_task_type import BackgroundTaskType

T = TypeVar("T", bound="MessageBroadcastOut")


@attr.s(auto_attribs=True)
class MessageBroadcastOut:
    """
    Attributes:
        id (str):
        status (BackgroundTaskStatus):
        task (BackgroundTaskType):
    """

    id: str
    status: BackgroundTaskStatus
    task: BackgroundTaskType
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        id = self.id
        status = self.status.value

        task = self.task.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "id": id,
                "status": status,
                "task": task,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        id = d.pop("id")

        status = BackgroundTaskStatus(d.pop("status"))

        task = BackgroundTaskType(d.pop("task"))

        message_broadcast_out = cls(
            id=id,
            status=status,
            task=task,
        )

        message_broadcast_out.additional_properties = d
        return message_broadcast_out

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
