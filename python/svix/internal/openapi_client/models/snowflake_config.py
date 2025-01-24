# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="SnowflakeConfig")


@attr.s(auto_attribs=True)
class SnowflakeConfig:
    """Configuration parameters for defining a Snowflake sink.

    Attributes:
        account_identifier (str): Snowflake account identifier, which includes both the organization and account IDs
            separated by a hyphen.
        private_key (str): PEM-encoded private key used for signing token-based requests to the Snowflake API.

            Beginning/end delimiters are not required.
        user_id (str): The Snowflake user id.
        db_name (Union[Unset, str]): Database name.

            Only required if not using transformations.
        schema_name (Union[Unset, str]): Schema name.

            Only required if not using transformations.
        table_name (Union[Unset, str]): Table name.

            Only required if not using transformations.
    """

    account_identifier: str
    private_key: str
    user_id: str
    db_name: Union[Unset, str] = UNSET
    schema_name: Union[Unset, str] = UNSET
    table_name: Union[Unset, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        account_identifier = self.account_identifier
        private_key = self.private_key
        user_id = self.user_id
        db_name = self.db_name
        schema_name = self.schema_name
        table_name = self.table_name

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "accountIdentifier": account_identifier,
                "privateKey": private_key,
                "userId": user_id,
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
        account_identifier = d.pop("accountIdentifier")

        private_key = d.pop("privateKey")

        user_id = d.pop("userId")

        db_name = d.pop("dbName", UNSET)

        schema_name = d.pop("schemaName", UNSET)

        table_name = d.pop("tableName", UNSET)

        snowflake_config = cls(
            account_identifier=account_identifier,
            private_key=private_key,
            user_id=user_id,
            db_name=db_name,
            schema_name=schema_name,
            table_name=table_name,
        )

        snowflake_config.additional_properties = d
        return snowflake_config

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
