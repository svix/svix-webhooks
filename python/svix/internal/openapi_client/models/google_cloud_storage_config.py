# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="GoogleCloudStorageConfig")


@attr.s(auto_attribs=True)
class GoogleCloudStorageConfig:
    """Configuration for a Google Cloud Storage sink.

    Write stream events into the named bucket using the supplied Google Cloud credentials.

        Attributes:
            bucket (str):
            credentials (str): Google Cloud Credentials JSON Object as a string.
    """

    bucket: str
    credentials: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        bucket = self.bucket
        credentials = self.credentials

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "bucket": bucket,
                "credentials": credentials,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        bucket = d.pop("bucket")

        credentials = d.pop("credentials")

        google_cloud_storage_config = cls(
            bucket=bucket,
            credentials=credentials,
        )

        google_cloud_storage_config.additional_properties = d
        return google_cloud_storage_config

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
