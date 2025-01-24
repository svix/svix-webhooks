# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="S3Config")


@attr.s(auto_attribs=True)
class S3Config:
    """
    Attributes:
        access_key_id (str):
        bucket (str):
        region (str):
        secret_access_key (str):
    """

    access_key_id: str
    bucket: str
    region: str
    secret_access_key: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        access_key_id = self.access_key_id
        bucket = self.bucket
        region = self.region
        secret_access_key = self.secret_access_key

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "accessKeyId": access_key_id,
                "bucket": bucket,
                "region": region,
                "secretAccessKey": secret_access_key,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        access_key_id = d.pop("accessKeyId")

        bucket = d.pop("bucket")

        region = d.pop("region")

        secret_access_key = d.pop("secretAccessKey")

        s3_config = cls(
            access_key_id=access_key_id,
            bucket=bucket,
            region=region,
            secret_access_key=secret_access_key,
        )

        s3_config.additional_properties = d
        return s3_config

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
