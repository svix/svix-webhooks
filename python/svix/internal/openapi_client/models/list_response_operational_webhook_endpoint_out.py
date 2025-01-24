# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.operational_webhook_endpoint_out import OperationalWebhookEndpointOut


T = TypeVar("T", bound="ListResponseOperationalWebhookEndpointOut")


@attr.s(auto_attribs=True)
class ListResponseOperationalWebhookEndpointOut:
    """
    Attributes:
        data (List['OperationalWebhookEndpointOut']):
        done (bool):
        iterator (Optional[str]):  Example: iterator.
        prev_iterator (Union[Unset, None, str]):  Example: -iterator.
    """

    data: List["OperationalWebhookEndpointOut"]
    done: bool
    iterator: Optional[str]
    prev_iterator: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        data = []
        for data_item_data in self.data:
            data_item = data_item_data.to_dict()

            data.append(data_item)

        done = self.done
        iterator = self.iterator
        prev_iterator = self.prev_iterator

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "data": data,
                "done": done,
                "iterator": iterator,
            }
        )
        if prev_iterator is not UNSET:
            field_dict["prevIterator"] = prev_iterator

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.operational_webhook_endpoint_out import OperationalWebhookEndpointOut

        d = src_dict.copy()
        data = []
        _data = d.pop("data")
        for data_item_data in _data:
            data_item = OperationalWebhookEndpointOut.from_dict(data_item_data)

            data.append(data_item)

        done = d.pop("done")

        iterator = d.pop("iterator")

        prev_iterator = d.pop("prevIterator", UNSET)

        list_response_operational_webhook_endpoint_out = cls(
            data=data,
            done=done,
            iterator=iterator,
            prev_iterator=prev_iterator,
        )

        list_response_operational_webhook_endpoint_out.additional_properties = d
        return list_response_operational_webhook_endpoint_out

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
