# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..models.oauth_2_auth_method_in import Oauth2AuthMethodIn
from ..models.oauth_2_grant_type_in import Oauth2GrantTypeIn
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.client_secret_jwt_params_in import ClientSecretJwtParamsIn
    from ..models.endpoint_oauth_config_in_extra_params import EndpointOauthConfigInExtraParams


T = TypeVar("T", bound="EndpointOauthConfigIn")


@attr.s(auto_attribs=True)
class EndpointOauthConfigIn:
    """
    Attributes:
        auth_method (Oauth2AuthMethodIn): The method used for authenticating to the OAuth authorization server.

            `clientSecretJwt` will construct a JWT used for authentication with the oauth authorization server. This method
            is less commonly used and may not be supported by all oauth providers. `clientSecretBasic` will authenticate to
            the oauth authorization server using an `Authorization` header with the client secret as the value. This is the
            most common means of authentication. `clientSecretPost` will authenticate to the oauth authorization server by
            passing the client secret in a `client_secret` field in the request body. This method may not be supported by
            all oauth providers, and in general `clientSecretBasic` should be preferred.
        client_id (str): The client ID. Required for all authentication types.
        grant_type (Oauth2GrantTypeIn):
        token_url (str): The URL of the authorization server.
        client_secret (Union[Unset, None, str]): Optional client secret. This is only used for `clientSecretBasic` and
            `clientSecretPost`.

            For `clientSecretBasic`, the secret will be appended to the `Authorization` header. For `clientSecretPost`, this
            will be added to the body in a `client_secret` parameter.
        extra_params (Union[Unset, None, EndpointOauthConfigInExtraParams]): Extra parameters added to the request body
            as key-value pairs.
        jwt_params (Union[Unset, ClientSecretJwtParamsIn]):
        refresh_token (Union[Unset, None, str]): For `refreshToken` grant type.
        scopes (Union[Unset, None, List[str]]): Optional OAuth scopes added to the request body.
    """

    auth_method: Oauth2AuthMethodIn
    client_id: str
    grant_type: Oauth2GrantTypeIn
    token_url: str
    client_secret: Union[Unset, None, str] = UNSET
    extra_params: Union[Unset, None, "EndpointOauthConfigInExtraParams"] = UNSET
    jwt_params: Union[Unset, "ClientSecretJwtParamsIn"] = UNSET
    refresh_token: Union[Unset, None, str] = UNSET
    scopes: Union[Unset, None, List[str]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        auth_method = self.auth_method.value

        client_id = self.client_id
        grant_type = self.grant_type.value

        token_url = self.token_url
        client_secret = self.client_secret
        extra_params = self.extra_params
        jwt_params: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.jwt_params, Unset):
            jwt_params = self.jwt_params.to_dict()

        refresh_token = self.refresh_token
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
                "authMethod": auth_method,
                "clientId": client_id,
                "grantType": grant_type,
                "tokenUrl": token_url,
            }
        )
        if client_secret is not UNSET:
            field_dict["clientSecret"] = client_secret
        if extra_params is not UNSET:
            field_dict["extraParams"] = extra_params
        if jwt_params is not UNSET:
            field_dict["jwtParams"] = jwt_params
        if refresh_token is not UNSET:
            field_dict["refreshToken"] = refresh_token
        if scopes is not UNSET:
            field_dict["scopes"] = scopes

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.client_secret_jwt_params_in import ClientSecretJwtParamsIn

        d = src_dict.copy()
        auth_method = Oauth2AuthMethodIn(d.pop("authMethod"))

        client_id = d.pop("clientId")

        grant_type = Oauth2GrantTypeIn(d.pop("grantType"))

        token_url = d.pop("tokenUrl")

        client_secret = d.pop("clientSecret", UNSET)

        extra_params = d.pop("extraParams", UNSET)

        _jwt_params = d.pop("jwtParams", UNSET)
        jwt_params: Union[Unset, ClientSecretJwtParamsIn]
        if isinstance(_jwt_params, Unset):
            jwt_params = UNSET
        else:
            jwt_params = ClientSecretJwtParamsIn.from_dict(_jwt_params)

        refresh_token = d.pop("refreshToken", UNSET)

        scopes = cast(List[str], d.pop("scopes", UNSET))

        endpoint_oauth_config_in = cls(
            auth_method=auth_method,
            client_id=client_id,
            grant_type=grant_type,
            token_url=token_url,
            client_secret=client_secret,
            extra_params=extra_params,
            jwt_params=jwt_params,
            refresh_token=refresh_token,
            scopes=scopes,
        )

        endpoint_oauth_config_in.additional_properties = d
        return endpoint_oauth_config_in

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
