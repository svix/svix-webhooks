# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.event_type_import_open_api_in_spec import EventTypeImportOpenApiInSpec


T = TypeVar("T", bound="EventTypeImportOpenApiIn")


@attr.s(auto_attribs=True)
class EventTypeImportOpenApiIn:
    """Import a list of event types from webhooks defined in an OpenAPI spec.

    The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`)
    which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad
    Request**.

        Attributes:
            dry_run (Union[Unset, bool]): If `true`, return the event types that would be modified without actually
                modifying them.
            spec (Union[Unset, None, EventTypeImportOpenApiInSpec]): A pre-parsed JSON spec. Example: {'info': {'title':
                'Webhook Example', 'version': '1.0.0'}, 'openapi': '3.1.0', 'webhooks': {'pet.new': {'post': {'requestBody':
                {'content': {'application/json': {'schema': {'properties': {'id': {'format': 'int64', 'type': 'integer'},
                'name': {'type': 'string'}, 'tag': {'type': 'string'}}, 'required': ['id', 'name']}}}, 'description':
                'Information about a new pet in the system'}, 'responses': {'200': {'description': 'Return a 200 status to
                indicate that the data was received successfully'}}}}}}.
            spec_raw (Union[Unset, None, str]): A string, parsed by the server as YAML or JSON. Example: {'info': {'title':
                'Webhook Example', 'version': '1.0.0'}, 'openapi': '3.1.0', 'webhooks': {'pet.new': {'post': {'requestBody':
                {'content': {'application/json': {'schema': {'properties': {'id': {'format': 'int64', 'type': 'integer'},
                'name': {'type': 'string'}, 'tag': {'type': 'string'}}, 'required': ['id', 'name']}}}, 'description':
                'Information about a new pet in the system'}, 'responses': {'200': {'description': 'Return a 200 status to
                indicate that the data was received successfully'}}}}}}.
    """

    dry_run: Union[Unset, bool] = False
    spec: Union[Unset, None, "EventTypeImportOpenApiInSpec"] = UNSET
    spec_raw: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        dry_run = self.dry_run
        spec = self.spec
        spec_raw = self.spec_raw

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if dry_run is not UNSET:
            field_dict["dry_run"] = dry_run
        if spec is not UNSET:
            field_dict["spec"] = spec
        if spec_raw is not UNSET:
            field_dict["specRaw"] = spec_raw

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        dry_run = d.pop("dry_run", UNSET)

        spec = d.pop("spec", UNSET)

        spec_raw = d.pop("specRaw", UNSET)

        event_type_import_open_api_in = cls(
            dry_run=dry_run,
            spec=spec,
            spec_raw=spec_raw,
        )

        event_type_import_open_api_in.additional_properties = d
        return event_type_import_open_api_in

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
