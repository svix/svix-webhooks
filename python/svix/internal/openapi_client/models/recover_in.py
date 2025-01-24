# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

T = TypeVar("T", bound="RecoverIn")


@attr.s(auto_attribs=True)
class RecoverIn:
    """
    Attributes:
        since (datetime.datetime):
        until (Union[Unset, None, datetime.datetime]):
    """

    since: datetime.datetime
    until: Union[Unset, None, datetime.datetime] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        since = self.since.isoformat()

        until: Union[Unset, None, str] = UNSET
        if not isinstance(self.until, Unset):
            until = self.until.isoformat() if self.until else None

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "since": since,
            }
        )
        if until is not UNSET:
            field_dict["until"] = until

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        since = isoparse(d.pop("since"))

        _until = d.pop("until", UNSET)
        until: Union[Unset, None, datetime.datetime]
        if _until is None:
            until = None
        elif isinstance(_until, Unset):
            until = UNSET
        else:
            until = isoparse(_until)

        recover_in = cls(
            since=since,
            until=until,
        )

        recover_in.additional_properties = d
        return recover_in

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
