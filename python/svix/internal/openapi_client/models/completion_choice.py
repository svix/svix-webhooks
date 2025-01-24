# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.completion_message import CompletionMessage


T = TypeVar("T", bound="CompletionChoice")


@attr.s(auto_attribs=True)
class CompletionChoice:
    """
    Attributes:
        finish_reason (str):
        index (int):
        message (CompletionMessage):
    """

    finish_reason: str
    index: int
    message: "CompletionMessage"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        finish_reason = self.finish_reason
        index = self.index
        message = self.message.to_dict()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "finish_reason": finish_reason,
                "index": index,
                "message": message,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.completion_message import CompletionMessage

        d = src_dict.copy()
        finish_reason = d.pop("finish_reason")

        index = d.pop("index")

        message = CompletionMessage.from_dict(d.pop("message"))

        completion_choice = cls(
            finish_reason=finish_reason,
            index=index,
            message=message,
        )

        completion_choice.additional_properties = d
        return completion_choice

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
