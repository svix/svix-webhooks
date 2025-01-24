# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.completion_choice import CompletionChoice


T = TypeVar("T", bound="GenerateOut")


@attr.s(auto_attribs=True)
class GenerateOut:
    """
    Attributes:
        choices (List['CompletionChoice']):
        created (int):
        id (str):
        model (str):
        object_ (str):
    """

    choices: List["CompletionChoice"]
    created: int
    id: str
    model: str
    object_: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        choices = []
        for choices_item_data in self.choices:
            choices_item = choices_item_data.to_dict()

            choices.append(choices_item)

        created = self.created
        id = self.id
        model = self.model
        object_ = self.object_

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "choices": choices,
                "created": created,
                "id": id,
                "model": model,
                "object": object_,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.completion_choice import CompletionChoice

        d = src_dict.copy()
        choices = []
        _choices = d.pop("choices")
        for choices_item_data in _choices:
            choices_item = CompletionChoice.from_dict(choices_item_data)

            choices.append(choices_item)

        created = d.pop("created")

        id = d.pop("id")

        model = d.pop("model")

        object_ = d.pop("object")

        generate_out = cls(
            choices=choices,
            created=created,
            id=id,
            model=model,
            object_=object_,
        )

        generate_out.additional_properties = d
        return generate_out

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
