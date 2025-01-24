# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.oauth_jws_signing_algorithm import OauthJwsSigningAlgorithm
from ..types import UNSET, Unset

T = TypeVar("T", bound="ClientSecretJwtParamsIn")


@attr.s(auto_attribs=True)
class ClientSecretJwtParamsIn:
    """
    Attributes:
        secret_base_64 (str): The base64-encoded secret used for signing the JWT.
        signing_algorithm (OauthJwsSigningAlgorithm):
        secret_id (Union[Unset, None, str]): Optional secret identifier. If supplied, this will be populated in the JWT
            header in the `kid` field.
        token_expiry_secs (Union[Unset, None, int]): Optional number of seconds after which the JWT should expire.
            Defaults to 300 seconds.
    """

    secret_base_64: str
    signing_algorithm: OauthJwsSigningAlgorithm
    secret_id: Union[Unset, None, str] = UNSET
    token_expiry_secs: Union[Unset, None, int] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        secret_base_64 = self.secret_base_64
        signing_algorithm = self.signing_algorithm.value

        secret_id = self.secret_id
        token_expiry_secs = self.token_expiry_secs

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "secretBase64": secret_base_64,
                "signingAlgorithm": signing_algorithm,
            }
        )
        if secret_id is not UNSET:
            field_dict["secretId"] = secret_id
        if token_expiry_secs is not UNSET:
            field_dict["tokenExpirySecs"] = token_expiry_secs

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        secret_base_64 = d.pop("secretBase64")

        signing_algorithm = OauthJwsSigningAlgorithm(d.pop("signingAlgorithm"))

        secret_id = d.pop("secretId", UNSET)

        token_expiry_secs = d.pop("tokenExpirySecs", UNSET)

        client_secret_jwt_params_in = cls(
            secret_base_64=secret_base_64,
            signing_algorithm=signing_algorithm,
            secret_id=secret_id,
            token_expiry_secs=token_expiry_secs,
        )

        client_secret_jwt_params_in.additional_properties = d
        return client_secret_jwt_params_in

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
