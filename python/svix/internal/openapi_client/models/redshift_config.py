# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="RedshiftConfig")


@attr.s(auto_attribs=True)
class RedshiftConfig:
    """Configuration parameters for defining a Redshift sink.

    Attributes:
        access_key_id (str):
        cluster_identifier (str):
        db_user (str):
        region (str):
        secret_access_key (str):
        db_name (Union[Unset, str]): Database name.

            Only required if not using transformations.
        schema_name (Union[Unset, None, str]): Schema name.

            Only used if not using transformations.
        table_name (Union[Unset, str]): Table name.

            Only required if not using transformations.
    """

    access_key_id: str
    cluster_identifier: str
    db_user: str
    region: str
    secret_access_key: str
    db_name: Union[Unset, str] = UNSET
    schema_name: Union[Unset, None, str] = UNSET
    table_name: Union[Unset, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        access_key_id = self.access_key_id
        cluster_identifier = self.cluster_identifier
        db_user = self.db_user
        region = self.region
        secret_access_key = self.secret_access_key
        db_name = self.db_name
        schema_name = self.schema_name
        table_name = self.table_name

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "accessKeyId": access_key_id,
                "clusterIdentifier": cluster_identifier,
                "dbUser": db_user,
                "region": region,
                "secretAccessKey": secret_access_key,
            }
        )
        if db_name is not UNSET:
            field_dict["dbName"] = db_name
        if schema_name is not UNSET:
            field_dict["schemaName"] = schema_name
        if table_name is not UNSET:
            field_dict["tableName"] = table_name

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        access_key_id = d.pop("accessKeyId")

        cluster_identifier = d.pop("clusterIdentifier")

        db_user = d.pop("dbUser")

        region = d.pop("region")

        secret_access_key = d.pop("secretAccessKey")

        db_name = d.pop("dbName", UNSET)

        schema_name = d.pop("schemaName", UNSET)

        table_name = d.pop("tableName", UNSET)

        redshift_config = cls(
            access_key_id=access_key_id,
            cluster_identifier=cluster_identifier,
            db_user=db_user,
            region=region,
            secret_access_key=secret_access_key,
            db_name=db_name,
            schema_name=schema_name,
            table_name=table_name,
        )

        redshift_config.additional_properties = d
        return redshift_config

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
