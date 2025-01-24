# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.stream_sink_out_type_0 import StreamSinkOutType0
    from ..models.stream_sink_out_type_1 import StreamSinkOutType1
    from ..models.stream_sink_out_type_2 import StreamSinkOutType2
    from ..models.stream_sink_out_type_3 import StreamSinkOutType3
    from ..models.stream_sink_out_type_4 import StreamSinkOutType4
    from ..models.stream_sink_out_type_5 import StreamSinkOutType5
    from ..models.stream_sink_out_type_6 import StreamSinkOutType6
    from ..models.stream_sink_out_type_7 import StreamSinkOutType7


T = TypeVar("T", bound="ListResponseStreamSinkOut")


@attr.s(auto_attribs=True)
class ListResponseStreamSinkOut:
    """
    Attributes:
        data (List[Union['StreamSinkOutType0', 'StreamSinkOutType1', 'StreamSinkOutType2', 'StreamSinkOutType3',
            'StreamSinkOutType4', 'StreamSinkOutType5', 'StreamSinkOutType6', 'StreamSinkOutType7']]):
        done (bool):
        iterator (Optional[str]):  Example: iterator.
        prev_iterator (Union[Unset, None, str]):  Example: -iterator.
    """

    data: List[
        Union[
            "StreamSinkOutType0",
            "StreamSinkOutType1",
            "StreamSinkOutType2",
            "StreamSinkOutType3",
            "StreamSinkOutType4",
            "StreamSinkOutType5",
            "StreamSinkOutType6",
            "StreamSinkOutType7",
        ]
    ]
    done: bool
    iterator: Optional[str]
    prev_iterator: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        from ..models.stream_sink_out_type_0 import StreamSinkOutType0
        from ..models.stream_sink_out_type_1 import StreamSinkOutType1
        from ..models.stream_sink_out_type_2 import StreamSinkOutType2
        from ..models.stream_sink_out_type_3 import StreamSinkOutType3
        from ..models.stream_sink_out_type_4 import StreamSinkOutType4
        from ..models.stream_sink_out_type_5 import StreamSinkOutType5
        from ..models.stream_sink_out_type_6 import StreamSinkOutType6

        data = []
        for data_item_data in self.data:
            data_item: Dict[str, Any]

            if isinstance(data_item_data, StreamSinkOutType0):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType1):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType2):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType3):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType4):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType5):
                data_item = data_item_data.to_dict()

            elif isinstance(data_item_data, StreamSinkOutType6):
                data_item = data_item_data.to_dict()

            else:
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
        from ..models.stream_sink_out_type_0 import StreamSinkOutType0
        from ..models.stream_sink_out_type_1 import StreamSinkOutType1
        from ..models.stream_sink_out_type_2 import StreamSinkOutType2
        from ..models.stream_sink_out_type_3 import StreamSinkOutType3
        from ..models.stream_sink_out_type_4 import StreamSinkOutType4
        from ..models.stream_sink_out_type_5 import StreamSinkOutType5
        from ..models.stream_sink_out_type_6 import StreamSinkOutType6
        from ..models.stream_sink_out_type_7 import StreamSinkOutType7

        d = src_dict.copy()
        data = []
        _data = d.pop("data")
        for data_item_data in _data:

            def _parse_data_item(
                data: object,
            ) -> Union[
                "StreamSinkOutType0",
                "StreamSinkOutType1",
                "StreamSinkOutType2",
                "StreamSinkOutType3",
                "StreamSinkOutType4",
                "StreamSinkOutType5",
                "StreamSinkOutType6",
                "StreamSinkOutType7",
            ]:
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_0 = StreamSinkOutType0.from_dict(data)

                    return componentsschemas_stream_sink_out_type_0
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_1 = StreamSinkOutType1.from_dict(data)

                    return componentsschemas_stream_sink_out_type_1
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_2 = StreamSinkOutType2.from_dict(data)

                    return componentsschemas_stream_sink_out_type_2
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_3 = StreamSinkOutType3.from_dict(data)

                    return componentsschemas_stream_sink_out_type_3
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_4 = StreamSinkOutType4.from_dict(data)

                    return componentsschemas_stream_sink_out_type_4
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_5 = StreamSinkOutType5.from_dict(data)

                    return componentsschemas_stream_sink_out_type_5
                except:  # noqa: E722
                    pass
                try:
                    if not isinstance(data, dict):
                        raise TypeError()
                    componentsschemas_stream_sink_out_type_6 = StreamSinkOutType6.from_dict(data)

                    return componentsschemas_stream_sink_out_type_6
                except:  # noqa: E722
                    pass
                if not isinstance(data, dict):
                    raise TypeError()
                componentsschemas_stream_sink_out_type_7 = StreamSinkOutType7.from_dict(data)

                return componentsschemas_stream_sink_out_type_7

            data_item = _parse_data_item(data_item_data)

            data.append(data_item)

        done = d.pop("done")

        iterator = d.pop("iterator")

        prev_iterator = d.pop("prevIterator", UNSET)

        list_response_stream_sink_out = cls(
            data=data,
            done=done,
            iterator=iterator,
            prev_iterator=prev_iterator,
        )

        list_response_stream_sink_out.additional_properties = d
        return list_response_stream_sink_out

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
