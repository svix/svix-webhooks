# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, cast

import attr

from ..models.background_task_status import BackgroundTaskStatus
from ..models.background_task_type import BackgroundTaskType

T = TypeVar("T", bound="AppUsageStatsOut")


@attr.s(auto_attribs=True)
class AppUsageStatsOut:
    """
    Attributes:
        id (str):
        status (BackgroundTaskStatus):
        task (BackgroundTaskType):
        unresolved_app_ids (List[str]): Any app IDs or UIDs received in the request that weren't found.

            Stats will be produced for all the others.
    """

    id: str
    status: BackgroundTaskStatus
    task: BackgroundTaskType
    unresolved_app_ids: List[str]
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        id = self.id
        status = self.status.value

        task = self.task.value

        unresolved_app_ids = self.unresolved_app_ids

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "id": id,
                "status": status,
                "task": task,
                "unresolvedAppIds": unresolved_app_ids,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        id = d.pop("id")

        status = BackgroundTaskStatus(d.pop("status"))

        task = BackgroundTaskType(d.pop("task"))

        unresolved_app_ids = cast(List[str], d.pop("unresolvedAppIds"))

        app_usage_stats_out = cls(
            id=id,
            status=status,
            task=task,
            unresolved_app_ids=unresolved_app_ids,
        )

        app_usage_stats_out.additional_properties = d
        return app_usage_stats_out

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
