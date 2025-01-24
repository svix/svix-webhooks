# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

T = TypeVar("T", bound="AuthTokenOut")


@attr.s(auto_attribs=True)
class AuthTokenOut:
    """
    Attributes:
        created_at (datetime.datetime):
        id (str): The key's ID Example: key_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        token (str):
        expires_at (Union[Unset, None, datetime.datetime]):
        name (Union[Unset, None, str]):
        scopes (Union[Unset, None, List[str]]):
    """

    created_at: datetime.datetime
    id: str
    token: str
    expires_at: Union[Unset, None, datetime.datetime] = UNSET
    name: Union[Unset, None, str] = UNSET
    scopes: Union[Unset, None, List[str]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        id = self.id
        token = self.token
        expires_at: Union[Unset, None, str] = UNSET
        if not isinstance(self.expires_at, Unset):
            expires_at = self.expires_at.isoformat() if self.expires_at else None

        name = self.name
        scopes: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.scopes, Unset):
            if self.scopes is None:
                scopes = None
            else:
                scopes = self.scopes

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "id": id,
                "token": token,
            }
        )
        if expires_at is not UNSET:
            field_dict["expiresAt"] = expires_at
        if name is not UNSET:
            field_dict["name"] = name
        if scopes is not UNSET:
            field_dict["scopes"] = scopes

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        id = d.pop("id")

        token = d.pop("token")

        _expires_at = d.pop("expiresAt", UNSET)
        expires_at: Union[Unset, None, datetime.datetime]
        if _expires_at is None:
            expires_at = None
        elif isinstance(_expires_at, Unset):
            expires_at = UNSET
        else:
            expires_at = isoparse(_expires_at)

        name = d.pop("name", UNSET)

        scopes = cast(List[str], d.pop("scopes", UNSET))

        auth_token_out = cls(
            created_at=created_at,
            id=id,
            token=token,
            expires_at=expires_at,
            name=name,
            scopes=scopes,
        )

        auth_token_out.additional_properties = d
        return auth_token_out

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
