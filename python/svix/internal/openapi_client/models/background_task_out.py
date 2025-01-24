# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.background_task_status import BackgroundTaskStatus
from ..models.background_task_type import BackgroundTaskType

if TYPE_CHECKING:
    from ..models.background_task_data import BackgroundTaskData


T = TypeVar("T", bound="BackgroundTaskOut")


@attr.s(auto_attribs=True)
class BackgroundTaskOut:
    """
    Attributes:
        data (BackgroundTaskData):
        id (str):
        status (BackgroundTaskStatus):
        task (BackgroundTaskType):
    """

    data: "BackgroundTaskData"
    id: str
    status: BackgroundTaskStatus
    task: BackgroundTaskType
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        data = self.data
        id = self.id
        status = self.status.value

        task = self.task.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "data": data,
                "id": id,
                "status": status,
                "task": task,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        data = d.pop("data")

        id = d.pop("id")

        status = BackgroundTaskStatus(d.pop("status"))

        task = BackgroundTaskType(d.pop("task"))

        background_task_out = cls(
            data=data,
            id=id,
            status=status,
            task=task,
        )

        background_task_out.additional_properties = d
        return background_task_out

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
