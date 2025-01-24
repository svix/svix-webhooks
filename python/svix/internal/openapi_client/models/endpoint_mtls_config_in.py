# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="EndpointMtlsConfigIn")


@attr.s(auto_attribs=True)
class EndpointMtlsConfigIn:
    """
    Attributes:
        identity (str): A PEM encoded private key and X509 certificate to identify the webhook sender.
        server_ca_cert (Union[Unset, None, str]): A PEM encoded X509 certificate used to verify the webhook receiver's
            certificate.
    """

    identity: str
    server_ca_cert: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        identity = self.identity
        server_ca_cert = self.server_ca_cert

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "identity": identity,
            }
        )
        if server_ca_cert is not UNSET:
            field_dict["serverCaCert"] = server_ca_cert

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        identity = d.pop("identity")

        server_ca_cert = d.pop("serverCaCert", UNSET)

        endpoint_mtls_config_in = cls(
            identity=identity,
            server_ca_cert=server_ca_cert,
        )

        endpoint_mtls_config_in.additional_properties = d
        return endpoint_mtls_config_in

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
