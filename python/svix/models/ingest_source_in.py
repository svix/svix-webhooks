# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .adobe_sign_config import AdobeSignConfig
from .common import BaseModel
from .cron_config import CronConfig
from .docusign_config import DocusignConfig
from .github_config import GithubConfig
from .hubspot_config import HubspotConfig
from .segment_config import SegmentConfig
from .shopify_config import ShopifyConfig
from .slack_config import SlackConfig
from .stripe_config import StripeConfig
from .svix_config import SvixConfig
from .zoom_config import ZoomConfig


class IngestSourceIn(BaseModel):
    name: str

    uid: t.Optional[str] = None
    """The Source's UID."""

    type: t.Union[
        t.Literal["generic-webhook"],
        t.Literal["cron"],
        t.Literal["adobe-sign"],
        t.Literal["beehiiv"],
        t.Literal["brex"],
        t.Literal["clerk"],
        t.Literal["docusign"],
        t.Literal["github"],
        t.Literal["guesty"],
        t.Literal["hubspot"],
        t.Literal["incident-io"],
        t.Literal["lithic"],
        t.Literal["nash"],
        t.Literal["pleo"],
        t.Literal["replicate"],
        t.Literal["resend"],
        t.Literal["safebase"],
        t.Literal["sardine"],
        t.Literal["segment"],
        t.Literal["shopify"],
        t.Literal["slack"],
        t.Literal["stripe"],
        t.Literal["stych"],
        t.Literal["svix"],
        t.Literal["zoom"],
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        CronConfig,
        AdobeSignConfig,
        SvixConfig,
        DocusignConfig,
        GithubConfig,
        HubspotConfig,
        SegmentConfig,
        ShopifyConfig,
        SlackConfig,
        StripeConfig,
        ZoomConfig,
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
        elif output.type == "clerk":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "docusign":
            output.config = DocusignConfig.model_validate(data.get("config", {}))
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
        elif output.type == "pleo":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "replicate":
            output.config = SvixConfig.model_validate(data.get("config", {}))
        elif output.type == "resend":
            output.config = SvixConfig.model_validate(data.get("config", {}))
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
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
