# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .adobe_sign_config import AdobeSignConfig
from .airwallex_config import AirwallexConfig
from .checkbook_config import CheckbookConfig
from .common import BaseModel
from .cron_config import CronConfig
from .docusign_config import DocusignConfig
from .easypost_config import EasypostConfig
from .github_config import GithubConfig
from .hubspot_config import HubspotConfig
from .orum_io_config import OrumIoConfig
from .panda_doc_config import PandaDocConfig
from .port_io_config import PortIoConfig
from .rutter_config import RutterConfig
from .segment_config import SegmentConfig
from .shopify_config import ShopifyConfig
from .slack_config import SlackConfig
from .stripe_config import StripeConfig
from .svix_config import SvixConfig
from .telnyx_config import TelnyxConfig
from .vapi_config import VapiConfig
from .veriff_config import VeriffConfig
from .zoom_config import ZoomConfig


class IngestSourceIn(BaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: str

    uid: t.Optional[str] = None
    """The Source's UID."""

    type: t.Union[
        t.Literal["generic-webhook"],
        t.Literal["cron"],
        t.Literal["adobe-sign"],
        t.Literal["beehiiv"],
        t.Literal["brex"],
        t.Literal["checkbook"],
        t.Literal["clerk"],
        t.Literal["docusign"],
        t.Literal["easypost"],
        t.Literal["github"],
        t.Literal["guesty"],
        t.Literal["hubspot"],
        t.Literal["incident-io"],
        t.Literal["lithic"],
        t.Literal["nash"],
        t.Literal["orum-io"],
        t.Literal["panda-doc"],
        t.Literal["port-io"],
        t.Literal["pleo"],
        t.Literal["replicate"],
        t.Literal["resend"],
        t.Literal["rutter"],
        t.Literal["safebase"],
        t.Literal["sardine"],
        t.Literal["segment"],
        t.Literal["shopify"],
        t.Literal["slack"],
        t.Literal["stripe"],
        t.Literal["stych"],
        t.Literal["svix"],
        t.Literal["zoom"],
        t.Literal["telnyx"],
        t.Literal["vapi"],
        t.Literal["open-ai"],
        t.Literal["render"],
        t.Literal["veriff"],
        t.Literal["airwallex"],
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        CronConfig,
        AdobeSignConfig,
        SvixConfig,
        CheckbookConfig,
        DocusignConfig,
        EasypostConfig,
        GithubConfig,
        HubspotConfig,
        OrumIoConfig,
        PandaDocConfig,
        PortIoConfig,
        RutterConfig,
        SegmentConfig,
        ShopifyConfig,
        SlackConfig,
        StripeConfig,
        ZoomConfig,
        TelnyxConfig,
        VapiConfig,
        VeriffConfig,
        AirwallexConfig,
    ]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        output = handler(data)
        if output.type == "generic-webhook":
            output.config = data.get("config", {})
        elif output.type == "cron":
            output.config = CronConfig.model_validate(data.get("config", {}))
        elif output.type == "adobe-sign":
            output.config = AdobeSignConfig.model_validate(data.get("config", {}))
        elif output.type == "beehiiv":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "brex":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "checkbook":
            output.config = CheckbookConfig.model_validate(data.get("config", {}))
        elif output.type == "clerk":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "docusign":
            output.config = DocusignConfig.model_validate(data.get("config", {}))
        elif output.type == "easypost":
            output.config = EasypostConfig.model_validate(data.get("config", {}))
        elif output.type == "github":
            output.config = GithubConfig.model_validate(data.get("config", {}))
        elif output.type == "guesty":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "hubspot":
            output.config = HubspotConfig.model_validate(data.get("config", {}))
        elif output.type == "incident-io":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "lithic":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "nash":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "orum-io":
            output.config = OrumIoConfig.model_validate(data.get("config", {}))
        elif output.type == "panda-doc":
            output.config = PandaDocConfig.model_validate(data.get("config", {}))
        elif output.type == "port-io":
            output.config = PortIoConfig.model_validate(data.get("config", {}))
        elif output.type == "pleo":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "replicate":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "resend":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "rutter":
            output.config = RutterConfig.model_validate(data.get("config", {}))
        elif output.type == "safebase":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "sardine":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "segment":
            output.config = SegmentConfig.model_validate(data.get("config", {}))
        elif output.type == "shopify":
            output.config = ShopifyConfig.model_validate(data.get("config", {}))
        elif output.type == "slack":
            output.config = SlackConfig.model_validate(data.get("config", {}))
        elif output.type == "stripe":
            output.config = StripeConfig.model_validate(data.get("config", {}))
        elif output.type == "stych":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "svix":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "zoom":
            output.config = ZoomConfig.model_validate(data.get("config", {}))
        elif output.type == "telnyx":
            output.config = TelnyxConfig.model_validate(data.get("config", {}))
        elif output.type == "vapi":
            output.config = VapiConfig.model_validate(data.get("config", {}))
        elif output.type == "open-ai":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "render":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "veriff":
            output.config = VeriffConfig.model_validate(data.get("config", {}))
        elif output.type == "airwallex":
            output.config = AirwallexConfig.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
