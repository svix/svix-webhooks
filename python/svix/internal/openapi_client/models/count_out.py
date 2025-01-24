# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="CountOut")


@attr.s(auto_attribs=True)
class CountOut:
    """
    Attributes:
        approximated (bool): There's a ceiling to how many attempts we count. When the limit is reached, this will be
            `true` to indicate the actual count is higher than given.
        count (int): The count of attempts matching the query.
    """

    approximated: bool
    count: int
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        approximated = self.approximated
        count = self.count

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "approximated": approximated,
                "count": count,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        approximated = d.pop("approximated")

        count = d.pop("count")

        count_out = cls(
            approximated=approximated,
            count=count,
        )

        count_out.additional_properties = d
        return count_out

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
