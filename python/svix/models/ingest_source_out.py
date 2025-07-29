# this file is @generated
import typing as t
from datetime import datetime

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .adobe_sign_config_out import AdobeSignConfigOut
from .airwallex_config_out import AirwallexConfigOut
from .checkbook_config_out import CheckbookConfigOut
from .common import BaseModel
from .cron_config import CronConfig
from .docusign_config_out import DocusignConfigOut
from .easypost_config_out import EasypostConfigOut
from .github_config_out import GithubConfigOut
from .hubspot_config_out import HubspotConfigOut
from .orum_io_config_out import OrumIoConfigOut
from .panda_doc_config_out import PandaDocConfigOut
from .port_io_config_out import PortIoConfigOut
from .rutter_config_out import RutterConfigOut
from .segment_config_out import SegmentConfigOut
from .shopify_config_out import ShopifyConfigOut
from .slack_config_out import SlackConfigOut
from .stripe_config_out import StripeConfigOut
from .svix_config_out import SvixConfigOut
from .telnyx_config_out import TelnyxConfigOut
from .vapi_config_out import VapiConfigOut
from .veriff_config_out import VeriffConfigOut
from .zoom_config_out import ZoomConfigOut


class IngestSourceOut(BaseModel):
    created_at: datetime

    id: str
    """The Source's ID."""

    ingest_url: t.Optional[str] = None

    metadata: t.Dict[str, str]

    name: str

    uid: t.Optional[str] = None
    """The Source's UID."""

    updated_at: datetime

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
        AdobeSignConfigOut,
        SvixConfigOut,
        CheckbookConfigOut,
        DocusignConfigOut,
        EasypostConfigOut,
        GithubConfigOut,
        HubspotConfigOut,
        OrumIoConfigOut,
        PandaDocConfigOut,
        PortIoConfigOut,
        RutterConfigOut,
        SegmentConfigOut,
        ShopifyConfigOut,
        SlackConfigOut,
        StripeConfigOut,
        ZoomConfigOut,
        TelnyxConfigOut,
        VapiConfigOut,
        VeriffConfigOut,
        AirwallexConfigOut,
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
            output.config = AdobeSignConfigOut.model_validate(data.get("config", {}))
        elif output.type == "beehiiv":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "brex":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "checkbook":
            output.config = CheckbookConfigOut.model_validate(data.get("config", {}))
        elif output.type == "clerk":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "docusign":
            output.config = DocusignConfigOut.model_validate(data.get("config", {}))
        elif output.type == "easypost":
            output.config = EasypostConfigOut.model_validate(data.get("config", {}))
        elif output.type == "github":
            output.config = GithubConfigOut.model_validate(data.get("config", {}))
        elif output.type == "guesty":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "hubspot":
            output.config = HubspotConfigOut.model_validate(data.get("config", {}))
        elif output.type == "incident-io":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "lithic":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "nash":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "orum-io":
            output.config = OrumIoConfigOut.model_validate(data.get("config", {}))
        elif output.type == "panda-doc":
            output.config = PandaDocConfigOut.model_validate(data.get("config", {}))
        elif output.type == "port-io":
            output.config = PortIoConfigOut.model_validate(data.get("config", {}))
        elif output.type == "pleo":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "replicate":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "resend":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "rutter":
            output.config = RutterConfigOut.model_validate(data.get("config", {}))
        elif output.type == "safebase":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "sardine":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "segment":
            output.config = SegmentConfigOut.model_validate(data.get("config", {}))
        elif output.type == "shopify":
            output.config = ShopifyConfigOut.model_validate(data.get("config", {}))
        elif output.type == "slack":
            output.config = SlackConfigOut.model_validate(data.get("config", {}))
        elif output.type == "stripe":
            output.config = StripeConfigOut.model_validate(data.get("config", {}))
        elif output.type == "stych":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "svix":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "zoom":
            output.config = ZoomConfigOut.model_validate(data.get("config", {}))
        elif output.type == "telnyx":
            output.config = TelnyxConfigOut.model_validate(data.get("config", {}))
        elif output.type == "vapi":
            output.config = VapiConfigOut.model_validate(data.get("config", {}))
        elif output.type == "open-ai":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "render":
            output.config = SvixConfigOut.model_validate(data.get("config", {}))
        elif output.type == "veriff":
            output.config = VeriffConfigOut.model_validate(data.get("config", {}))
        elif output.type == "airwallex":
            output.config = AirwallexConfigOut.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
