# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="BigQueryConfig")


@attr.s(auto_attribs=True)
class BigQueryConfig:
    """Configuration for a Google Cloud BigQuery sink.

    Attributes:
        credentials (str): Google Cloud Credentials JSON Object as a string.
        dataset_id (str):
        project_id (str):
        table_id (str):
    """

    credentials: str
    dataset_id: str
    project_id: str
    table_id: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        credentials = self.credentials
        dataset_id = self.dataset_id
        project_id = self.project_id
        table_id = self.table_id

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "credentials": credentials,
                "datasetId": dataset_id,
                "projectId": project_id,
                "tableId": table_id,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        credentials = d.pop("credentials")

        dataset_id = d.pop("datasetId")

        project_id = d.pop("projectId")

        table_id = d.pop("tableId")

        big_query_config = cls(
            credentials=credentials,
            dataset_id=dataset_id,
            project_id=project_id,
            table_id=table_id,
        )

        big_query_config.additional_properties = d
        return big_query_config

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
