# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.duration import Duration


T = TypeVar("T", bound="RetryScheduleInOut")


@attr.s(auto_attribs=True)
class RetryScheduleInOut:
    """
    Attributes:
        retry_schedule (Union[Unset, List['Duration']]):
    """

    retry_schedule: Union[Unset, List["Duration"]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        retry_schedule: Union[Unset, List[Dict[str, Any]]] = UNSET
        if not isinstance(self.retry_schedule, Unset):
            retry_schedule = []
            for componentsschemas_retry_schedule_item_data in self.retry_schedule:
                componentsschemas_retry_schedule_item = componentsschemas_retry_schedule_item_data.to_dict()

                retry_schedule.append(componentsschemas_retry_schedule_item)

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if retry_schedule is not UNSET:
            field_dict["retrySchedule"] = retry_schedule

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.duration import Duration

        d = src_dict.copy()
        retry_schedule = []
        _retry_schedule = d.pop("retrySchedule", UNSET)
        for componentsschemas_retry_schedule_item_data in _retry_schedule or []:
            componentsschemas_retry_schedule_item = Duration.from_dict(componentsschemas_retry_schedule_item_data)

            retry_schedule.append(componentsschemas_retry_schedule_item)

        retry_schedule_in_out = cls(
            retry_schedule=retry_schedule,
        )

        retry_schedule_in_out.additional_properties = d
        return retry_schedule_in_out

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
