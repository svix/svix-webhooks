// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(IngestSourceOutConverter))]
    public class IngestSourceOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("ingestUrl")]
        public string? IngestUrl { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonIgnore]
        public required IngestSourceOutConfig Config { get; set; }

        [JsonProperty("type")]
        private string Type => Config.GetDiscriminator();

        [JsonProperty("config")]
        private object realConfig
        {
            get => Config.GetContent();
            set => Config.SetContent(value);
        }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestSourceOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  IngestUrl: ").Append(IngestUrl).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class IngestSourceOutConfig
    {
        private object _value;
        private readonly ConfigType _type;

        internal string GetDiscriminator()
        {
            var memberInfo = typeof(ConfigType).GetMember(_type.ToString());
            var enumMember = memberInfo[0]
                .GetCustomAttributes(false)
                .OfType<EnumMemberAttribute>()
                .FirstOrDefault();
            return enumMember?.Value ?? _type.ToString().ToLower();
        }

        internal void SetContent(object value)
        {
            _value = value;
        }

        public object GetContent()
        {
            return _value;
        }

        private IngestSourceOutConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static IngestSourceOutConfig GenericWebhook() =>
            new(new Dictionary<string, string>(), ConfigType.GenericWebhook);

        public static IngestSourceOutConfig Cron(CronConfig cronConfig) =>
            new(cronConfig, ConfigType.Cron);

        public static IngestSourceOutConfig AdobeSign(AdobeSignConfigOut adobeSignConfigOut) =>
            new(adobeSignConfigOut, ConfigType.AdobeSign);

        public static IngestSourceOutConfig Beehiiv(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Beehiiv);

        public static IngestSourceOutConfig Brex(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Brex);

        public static IngestSourceOutConfig Checkbook(CheckbookConfigOut checkbookConfigOut) =>
            new(checkbookConfigOut, ConfigType.Checkbook);

        public static IngestSourceOutConfig Clerk(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Clerk);

        public static IngestSourceOutConfig Docusign(DocusignConfigOut docusignConfigOut) =>
            new(docusignConfigOut, ConfigType.Docusign);

        public static IngestSourceOutConfig Easypost(EasypostConfigOut easypostConfigOut) =>
            new(easypostConfigOut, ConfigType.Easypost);

        public static IngestSourceOutConfig Github(GithubConfigOut githubConfigOut) =>
            new(githubConfigOut, ConfigType.Github);

        public static IngestSourceOutConfig Guesty(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Guesty);

        public static IngestSourceOutConfig Hubspot(HubspotConfigOut hubspotConfigOut) =>
            new(hubspotConfigOut, ConfigType.Hubspot);

        public static IngestSourceOutConfig IncidentIo(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.IncidentIo);

        public static IngestSourceOutConfig Lithic(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Lithic);

        public static IngestSourceOutConfig Meta(MetaConfigOut metaConfigOut) =>
            new(metaConfigOut, ConfigType.Meta);

        public static IngestSourceOutConfig Nango(NangoConfigOut nangoConfigOut) =>
            new(nangoConfigOut, ConfigType.Nango);

        public static IngestSourceOutConfig Nash(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Nash);

        public static IngestSourceOutConfig Openclaw(OpenClawConfigOut openClawConfigOut) =>
            new(openClawConfigOut, ConfigType.Openclaw);

        public static IngestSourceOutConfig OrumIo(OrumIoConfigOut orumIoConfigOut) =>
            new(orumIoConfigOut, ConfigType.OrumIo);

        public static IngestSourceOutConfig PandaDoc(PandaDocConfigOut pandaDocConfigOut) =>
            new(pandaDocConfigOut, ConfigType.PandaDoc);

        public static IngestSourceOutConfig PortIo(PortIoConfigOut portIoConfigOut) =>
            new(portIoConfigOut, ConfigType.PortIo);

        public static IngestSourceOutConfig PsiFi(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.PsiFi);

        public static IngestSourceOutConfig Pleo(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Pleo);

        public static IngestSourceOutConfig Replicate(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Replicate);

        public static IngestSourceOutConfig Resend(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Resend);

        public static IngestSourceOutConfig Rutter(RutterConfigOut rutterConfigOut) =>
            new(rutterConfigOut, ConfigType.Rutter);

        public static IngestSourceOutConfig Safebase(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Safebase);

        public static IngestSourceOutConfig Sardine(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Sardine);

        public static IngestSourceOutConfig Segment(SegmentConfigOut segmentConfigOut) =>
            new(segmentConfigOut, ConfigType.Segment);

        public static IngestSourceOutConfig Shopify(ShopifyConfigOut shopifyConfigOut) =>
            new(shopifyConfigOut, ConfigType.Shopify);

        public static IngestSourceOutConfig Slack(SlackConfigOut slackConfigOut) =>
            new(slackConfigOut, ConfigType.Slack);

        public static IngestSourceOutConfig Stripe(StripeConfigOut stripeConfigOut) =>
            new(stripeConfigOut, ConfigType.Stripe);

        public static IngestSourceOutConfig Stych(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Stych);

        public static IngestSourceOutConfig Svix(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Svix);

        public static IngestSourceOutConfig Zoom(ZoomConfigOut zoomConfigOut) =>
            new(zoomConfigOut, ConfigType.Zoom);

        public static IngestSourceOutConfig Tailscale(TailscaleConfigOut tailscaleConfigOut) =>
            new(tailscaleConfigOut, ConfigType.Tailscale);

        public static IngestSourceOutConfig Telnyx(TelnyxConfigOut telnyxConfigOut) =>
            new(telnyxConfigOut, ConfigType.Telnyx);

        public static IngestSourceOutConfig Vapi(VapiConfigOut vapiConfigOut) =>
            new(vapiConfigOut, ConfigType.Vapi);

        public static IngestSourceOutConfig OpenAi(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.OpenAi);

        public static IngestSourceOutConfig Render(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Render);

        public static IngestSourceOutConfig Veriff(VeriffConfigOut veriffConfigOut) =>
            new(veriffConfigOut, ConfigType.Veriff);

        public static IngestSourceOutConfig Airwallex(AirwallexConfigOut airwallexConfigOut) =>
            new(airwallexConfigOut, ConfigType.Airwallex);

        public static IngestSourceOutConfig Vgs(VgsConfigOut vgsConfigOut) =>
            new(vgsConfigOut, ConfigType.Vgs);

        private enum ConfigType
        {
            [EnumMember(Value = "generic-webhook")]
            GenericWebhook,

            [EnumMember(Value = "cron")]
            Cron,

            [EnumMember(Value = "adobe-sign")]
            AdobeSign,

            [EnumMember(Value = "beehiiv")]
            Beehiiv,

            [EnumMember(Value = "brex")]
            Brex,

            [EnumMember(Value = "checkbook")]
            Checkbook,

            [EnumMember(Value = "clerk")]
            Clerk,

            [EnumMember(Value = "docusign")]
            Docusign,

            [EnumMember(Value = "easypost")]
            Easypost,

            [EnumMember(Value = "github")]
            Github,

            [EnumMember(Value = "guesty")]
            Guesty,

            [EnumMember(Value = "hubspot")]
            Hubspot,

            [EnumMember(Value = "incident-io")]
            IncidentIo,

            [EnumMember(Value = "lithic")]
            Lithic,

            [EnumMember(Value = "meta")]
            Meta,

            [EnumMember(Value = "nango")]
            Nango,

            [EnumMember(Value = "nash")]
            Nash,

            [EnumMember(Value = "openclaw")]
            Openclaw,

            [EnumMember(Value = "orum-io")]
            OrumIo,

            [EnumMember(Value = "panda-doc")]
            PandaDoc,

            [EnumMember(Value = "port-io")]
            PortIo,

            [EnumMember(Value = "psi-fi")]
            PsiFi,

            [EnumMember(Value = "pleo")]
            Pleo,

            [EnumMember(Value = "replicate")]
            Replicate,

            [EnumMember(Value = "resend")]
            Resend,

            [EnumMember(Value = "rutter")]
            Rutter,

            [EnumMember(Value = "safebase")]
            Safebase,

            [EnumMember(Value = "sardine")]
            Sardine,

            [EnumMember(Value = "segment")]
            Segment,

            [EnumMember(Value = "shopify")]
            Shopify,

            [EnumMember(Value = "slack")]
            Slack,

            [EnumMember(Value = "stripe")]
            Stripe,

            [EnumMember(Value = "stych")]
            Stych,

            [EnumMember(Value = "svix")]
            Svix,

            [EnumMember(Value = "zoom")]
            Zoom,

            [EnumMember(Value = "tailscale")]
            Tailscale,

            [EnumMember(Value = "telnyx")]
            Telnyx,

            [EnumMember(Value = "vapi")]
            Vapi,

            [EnumMember(Value = "open-ai")]
            OpenAi,

            [EnumMember(Value = "render")]
            Render,

            [EnumMember(Value = "veriff")]
            Veriff,

            [EnumMember(Value = "airwallex")]
            Airwallex,

            [EnumMember(Value = "vgs")]
            Vgs,
        }

        public TResult Match<TResult>(
            Func<TResult> onGenericWebhook,
            Func<CronConfig, TResult> onCron,
            Func<AdobeSignConfigOut, TResult> onAdobeSign,
            Func<SvixConfigOut, TResult> onBeehiiv,
            Func<SvixConfigOut, TResult> onBrex,
            Func<CheckbookConfigOut, TResult> onCheckbook,
            Func<SvixConfigOut, TResult> onClerk,
            Func<DocusignConfigOut, TResult> onDocusign,
            Func<EasypostConfigOut, TResult> onEasypost,
            Func<GithubConfigOut, TResult> onGithub,
            Func<SvixConfigOut, TResult> onGuesty,
            Func<HubspotConfigOut, TResult> onHubspot,
            Func<SvixConfigOut, TResult> onIncidentIo,
            Func<SvixConfigOut, TResult> onLithic,
            Func<MetaConfigOut, TResult> onMeta,
            Func<NangoConfigOut, TResult> onNango,
            Func<SvixConfigOut, TResult> onNash,
            Func<OpenClawConfigOut, TResult> onOpenclaw,
            Func<OrumIoConfigOut, TResult> onOrumIo,
            Func<PandaDocConfigOut, TResult> onPandaDoc,
            Func<PortIoConfigOut, TResult> onPortIo,
            Func<SvixConfigOut, TResult> onPsiFi,
            Func<SvixConfigOut, TResult> onPleo,
            Func<SvixConfigOut, TResult> onReplicate,
            Func<SvixConfigOut, TResult> onResend,
            Func<RutterConfigOut, TResult> onRutter,
            Func<SvixConfigOut, TResult> onSafebase,
            Func<SvixConfigOut, TResult> onSardine,
            Func<SegmentConfigOut, TResult> onSegment,
            Func<ShopifyConfigOut, TResult> onShopify,
            Func<SlackConfigOut, TResult> onSlack,
            Func<StripeConfigOut, TResult> onStripe,
            Func<SvixConfigOut, TResult> onStych,
            Func<SvixConfigOut, TResult> onSvix,
            Func<ZoomConfigOut, TResult> onZoom,
            Func<TailscaleConfigOut, TResult> onTailscale,
            Func<TelnyxConfigOut, TResult> onTelnyx,
            Func<VapiConfigOut, TResult> onVapi,
            Func<SvixConfigOut, TResult> onOpenAi,
            Func<SvixConfigOut, TResult> onRender,
            Func<VeriffConfigOut, TResult> onVeriff,
            Func<AirwallexConfigOut, TResult> onAirwallex,
            Func<VgsConfigOut, TResult> onVgs
        )
        {
            return _type switch
            {
                ConfigType.GenericWebhook => onGenericWebhook(),
                ConfigType.Cron => onCron((CronConfig)_value),
                ConfigType.AdobeSign => onAdobeSign((AdobeSignConfigOut)_value),
                ConfigType.Beehiiv => onBeehiiv((SvixConfigOut)_value),
                ConfigType.Brex => onBrex((SvixConfigOut)_value),
                ConfigType.Checkbook => onCheckbook((CheckbookConfigOut)_value),
                ConfigType.Clerk => onClerk((SvixConfigOut)_value),
                ConfigType.Docusign => onDocusign((DocusignConfigOut)_value),
                ConfigType.Easypost => onEasypost((EasypostConfigOut)_value),
                ConfigType.Github => onGithub((GithubConfigOut)_value),
                ConfigType.Guesty => onGuesty((SvixConfigOut)_value),
                ConfigType.Hubspot => onHubspot((HubspotConfigOut)_value),
                ConfigType.IncidentIo => onIncidentIo((SvixConfigOut)_value),
                ConfigType.Lithic => onLithic((SvixConfigOut)_value),
                ConfigType.Meta => onMeta((MetaConfigOut)_value),
                ConfigType.Nango => onNango((NangoConfigOut)_value),
                ConfigType.Nash => onNash((SvixConfigOut)_value),
                ConfigType.Openclaw => onOpenclaw((OpenClawConfigOut)_value),
                ConfigType.OrumIo => onOrumIo((OrumIoConfigOut)_value),
                ConfigType.PandaDoc => onPandaDoc((PandaDocConfigOut)_value),
                ConfigType.PortIo => onPortIo((PortIoConfigOut)_value),
                ConfigType.PsiFi => onPsiFi((SvixConfigOut)_value),
                ConfigType.Pleo => onPleo((SvixConfigOut)_value),
                ConfigType.Replicate => onReplicate((SvixConfigOut)_value),
                ConfigType.Resend => onResend((SvixConfigOut)_value),
                ConfigType.Rutter => onRutter((RutterConfigOut)_value),
                ConfigType.Safebase => onSafebase((SvixConfigOut)_value),
                ConfigType.Sardine => onSardine((SvixConfigOut)_value),
                ConfigType.Segment => onSegment((SegmentConfigOut)_value),
                ConfigType.Shopify => onShopify((ShopifyConfigOut)_value),
                ConfigType.Slack => onSlack((SlackConfigOut)_value),
                ConfigType.Stripe => onStripe((StripeConfigOut)_value),
                ConfigType.Stych => onStych((SvixConfigOut)_value),
                ConfigType.Svix => onSvix((SvixConfigOut)_value),
                ConfigType.Zoom => onZoom((ZoomConfigOut)_value),
                ConfigType.Tailscale => onTailscale((TailscaleConfigOut)_value),
                ConfigType.Telnyx => onTelnyx((TelnyxConfigOut)_value),
                ConfigType.Vapi => onVapi((VapiConfigOut)_value),
                ConfigType.OpenAi => onOpenAi((SvixConfigOut)_value),
                ConfigType.Render => onRender((SvixConfigOut)_value),
                ConfigType.Veriff => onVeriff((VeriffConfigOut)_value),
                ConfigType.Airwallex => onAirwallex((AirwallexConfigOut)_value),
                ConfigType.Vgs => onVgs((VgsConfigOut)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action? onGenericWebhook = null,
            Action<CronConfig>? onCron = null,
            Action<AdobeSignConfigOut>? onAdobeSign = null,
            Action<SvixConfigOut>? onBeehiiv = null,
            Action<SvixConfigOut>? onBrex = null,
            Action<CheckbookConfigOut>? onCheckbook = null,
            Action<SvixConfigOut>? onClerk = null,
            Action<DocusignConfigOut>? onDocusign = null,
            Action<EasypostConfigOut>? onEasypost = null,
            Action<GithubConfigOut>? onGithub = null,
            Action<SvixConfigOut>? onGuesty = null,
            Action<HubspotConfigOut>? onHubspot = null,
            Action<SvixConfigOut>? onIncidentIo = null,
            Action<SvixConfigOut>? onLithic = null,
            Action<MetaConfigOut>? onMeta = null,
            Action<NangoConfigOut>? onNango = null,
            Action<SvixConfigOut>? onNash = null,
            Action<OpenClawConfigOut>? onOpenclaw = null,
            Action<OrumIoConfigOut>? onOrumIo = null,
            Action<PandaDocConfigOut>? onPandaDoc = null,
            Action<PortIoConfigOut>? onPortIo = null,
            Action<SvixConfigOut>? onPsiFi = null,
            Action<SvixConfigOut>? onPleo = null,
            Action<SvixConfigOut>? onReplicate = null,
            Action<SvixConfigOut>? onResend = null,
            Action<RutterConfigOut>? onRutter = null,
            Action<SvixConfigOut>? onSafebase = null,
            Action<SvixConfigOut>? onSardine = null,
            Action<SegmentConfigOut>? onSegment = null,
            Action<ShopifyConfigOut>? onShopify = null,
            Action<SlackConfigOut>? onSlack = null,
            Action<StripeConfigOut>? onStripe = null,
            Action<SvixConfigOut>? onStych = null,
            Action<SvixConfigOut>? onSvix = null,
            Action<ZoomConfigOut>? onZoom = null,
            Action<TailscaleConfigOut>? onTailscale = null,
            Action<TelnyxConfigOut>? onTelnyx = null,
            Action<VapiConfigOut>? onVapi = null,
            Action<SvixConfigOut>? onOpenAi = null,
            Action<SvixConfigOut>? onRender = null,
            Action<VeriffConfigOut>? onVeriff = null,
            Action<AirwallexConfigOut>? onAirwallex = null,
            Action<VgsConfigOut>? onVgs = null
        )
        {
            switch (_type)
            {
                case ConfigType.GenericWebhook:
                    if (onGenericWebhook != null)
                    {
                        onGenericWebhook();
                    }
                    break;
                case ConfigType.Cron:
                    if (onCron != null)
                    {
                        onCron((CronConfig)_value);
                    }
                    break;
                case ConfigType.AdobeSign:
                    if (onAdobeSign != null)
                    {
                        onAdobeSign((AdobeSignConfigOut)_value);
                    }
                    break;
                case ConfigType.Beehiiv:
                    if (onBeehiiv != null)
                    {
                        onBeehiiv((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Brex:
                    if (onBrex != null)
                    {
                        onBrex((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Checkbook:
                    if (onCheckbook != null)
                    {
                        onCheckbook((CheckbookConfigOut)_value);
                    }
                    break;
                case ConfigType.Clerk:
                    if (onClerk != null)
                    {
                        onClerk((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Docusign:
                    if (onDocusign != null)
                    {
                        onDocusign((DocusignConfigOut)_value);
                    }
                    break;
                case ConfigType.Easypost:
                    if (onEasypost != null)
                    {
                        onEasypost((EasypostConfigOut)_value);
                    }
                    break;
                case ConfigType.Github:
                    if (onGithub != null)
                    {
                        onGithub((GithubConfigOut)_value);
                    }
                    break;
                case ConfigType.Guesty:
                    if (onGuesty != null)
                    {
                        onGuesty((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Hubspot:
                    if (onHubspot != null)
                    {
                        onHubspot((HubspotConfigOut)_value);
                    }
                    break;
                case ConfigType.IncidentIo:
                    if (onIncidentIo != null)
                    {
                        onIncidentIo((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Lithic:
                    if (onLithic != null)
                    {
                        onLithic((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Meta:
                    if (onMeta != null)
                    {
                        onMeta((MetaConfigOut)_value);
                    }
                    break;
                case ConfigType.Nango:
                    if (onNango != null)
                    {
                        onNango((NangoConfigOut)_value);
                    }
                    break;
                case ConfigType.Nash:
                    if (onNash != null)
                    {
                        onNash((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Openclaw:
                    if (onOpenclaw != null)
                    {
                        onOpenclaw((OpenClawConfigOut)_value);
                    }
                    break;
                case ConfigType.OrumIo:
                    if (onOrumIo != null)
                    {
                        onOrumIo((OrumIoConfigOut)_value);
                    }
                    break;
                case ConfigType.PandaDoc:
                    if (onPandaDoc != null)
                    {
                        onPandaDoc((PandaDocConfigOut)_value);
                    }
                    break;
                case ConfigType.PortIo:
                    if (onPortIo != null)
                    {
                        onPortIo((PortIoConfigOut)_value);
                    }
                    break;
                case ConfigType.PsiFi:
                    if (onPsiFi != null)
                    {
                        onPsiFi((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Pleo:
                    if (onPleo != null)
                    {
                        onPleo((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Replicate:
                    if (onReplicate != null)
                    {
                        onReplicate((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Resend:
                    if (onResend != null)
                    {
                        onResend((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Rutter:
                    if (onRutter != null)
                    {
                        onRutter((RutterConfigOut)_value);
                    }
                    break;
                case ConfigType.Safebase:
                    if (onSafebase != null)
                    {
                        onSafebase((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Sardine:
                    if (onSardine != null)
                    {
                        onSardine((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Segment:
                    if (onSegment != null)
                    {
                        onSegment((SegmentConfigOut)_value);
                    }
                    break;
                case ConfigType.Shopify:
                    if (onShopify != null)
                    {
                        onShopify((ShopifyConfigOut)_value);
                    }
                    break;
                case ConfigType.Slack:
                    if (onSlack != null)
                    {
                        onSlack((SlackConfigOut)_value);
                    }
                    break;
                case ConfigType.Stripe:
                    if (onStripe != null)
                    {
                        onStripe((StripeConfigOut)_value);
                    }
                    break;
                case ConfigType.Stych:
                    if (onStych != null)
                    {
                        onStych((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Svix:
                    if (onSvix != null)
                    {
                        onSvix((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Zoom:
                    if (onZoom != null)
                    {
                        onZoom((ZoomConfigOut)_value);
                    }
                    break;
                case ConfigType.Tailscale:
                    if (onTailscale != null)
                    {
                        onTailscale((TailscaleConfigOut)_value);
                    }
                    break;
                case ConfigType.Telnyx:
                    if (onTelnyx != null)
                    {
                        onTelnyx((TelnyxConfigOut)_value);
                    }
                    break;
                case ConfigType.Vapi:
                    if (onVapi != null)
                    {
                        onVapi((VapiConfigOut)_value);
                    }
                    break;
                case ConfigType.OpenAi:
                    if (onOpenAi != null)
                    {
                        onOpenAi((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Render:
                    if (onRender != null)
                    {
                        onRender((SvixConfigOut)_value);
                    }
                    break;
                case ConfigType.Veriff:
                    if (onVeriff != null)
                    {
                        onVeriff((VeriffConfigOut)_value);
                    }
                    break;
                case ConfigType.Airwallex:
                    if (onAirwallex != null)
                    {
                        onAirwallex((AirwallexConfigOut)_value);
                    }
                    break;
                case ConfigType.Vgs:
                    if (onVgs != null)
                    {
                        onVgs((VgsConfigOut)_value);
                    }
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class IngestSourceOutSurrogate
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("ingestUrl")]
        public string? IngestUrl { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config")]
        public JObject? Config { get; set; }
    }

    public class IngestSourceOutConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(IngestSourceOut);
        }

        public override bool CanWrite
        {
            get { return false; }
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            // unreachable: CanWrite tells Newtonsoft not to call this method
            throw new NotImplementedException();
        }

        public override object ReadJson(
            JsonReader reader,
            Type objectType,
            object? existingValue,
            JsonSerializer serializer
        )
        {
            var surrogate =
                serializer.Deserialize<IngestSourceOutSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to IngestSourceOutSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), IngestSourceOutConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for IngestSourceOutConfig.config"
                );
            }

            IngestSourceOutConfig config = func((surrogate.Config, surrogate.Type));

            return new IngestSourceOut
            {
                Id = surrogate.Id,
                Uid = surrogate.Uid,
                Name = surrogate.Name,
                IngestUrl = surrogate.IngestUrl,
                CreatedAt = surrogate.CreatedAt,
                UpdatedAt = surrogate.UpdatedAt,
                Metadata = surrogate.Metadata,
                Config = config,
            };
        }

        private static T ToObj<T>((JObject, string) args)
        {
            var loadedObj =
                args.Item1.ToObject<T>()
                ?? throw new JsonSerializationException(
                    $"Failed to deserialize {args.Item2} config"
                );
            return loadedObj;
        }

        private readonly Dictionary<
            string,
            Func<(JObject, string), IngestSourceOutConfig>
        > typeMap = new()
        {
            ["generic-webhook"] = c => IngestSourceOutConfig.GenericWebhook(),
            ["cron"] = c => IngestSourceOutConfig.Cron(ToObj<CronConfig>(c)),
            ["adobe-sign"] = c => IngestSourceOutConfig.AdobeSign(ToObj<AdobeSignConfigOut>(c)),
            ["beehiiv"] = c => IngestSourceOutConfig.Beehiiv(ToObj<SvixConfigOut>(c)),
            ["brex"] = c => IngestSourceOutConfig.Brex(ToObj<SvixConfigOut>(c)),
            ["checkbook"] = c => IngestSourceOutConfig.Checkbook(ToObj<CheckbookConfigOut>(c)),
            ["clerk"] = c => IngestSourceOutConfig.Clerk(ToObj<SvixConfigOut>(c)),
            ["docusign"] = c => IngestSourceOutConfig.Docusign(ToObj<DocusignConfigOut>(c)),
            ["easypost"] = c => IngestSourceOutConfig.Easypost(ToObj<EasypostConfigOut>(c)),
            ["github"] = c => IngestSourceOutConfig.Github(ToObj<GithubConfigOut>(c)),
            ["guesty"] = c => IngestSourceOutConfig.Guesty(ToObj<SvixConfigOut>(c)),
            ["hubspot"] = c => IngestSourceOutConfig.Hubspot(ToObj<HubspotConfigOut>(c)),
            ["incident-io"] = c => IngestSourceOutConfig.IncidentIo(ToObj<SvixConfigOut>(c)),
            ["lithic"] = c => IngestSourceOutConfig.Lithic(ToObj<SvixConfigOut>(c)),
            ["meta"] = c => IngestSourceOutConfig.Meta(ToObj<MetaConfigOut>(c)),
            ["nango"] = c => IngestSourceOutConfig.Nango(ToObj<NangoConfigOut>(c)),
            ["nash"] = c => IngestSourceOutConfig.Nash(ToObj<SvixConfigOut>(c)),
            ["openclaw"] = c => IngestSourceOutConfig.Openclaw(ToObj<OpenClawConfigOut>(c)),
            ["orum-io"] = c => IngestSourceOutConfig.OrumIo(ToObj<OrumIoConfigOut>(c)),
            ["panda-doc"] = c => IngestSourceOutConfig.PandaDoc(ToObj<PandaDocConfigOut>(c)),
            ["port-io"] = c => IngestSourceOutConfig.PortIo(ToObj<PortIoConfigOut>(c)),
            ["psi-fi"] = c => IngestSourceOutConfig.PsiFi(ToObj<SvixConfigOut>(c)),
            ["pleo"] = c => IngestSourceOutConfig.Pleo(ToObj<SvixConfigOut>(c)),
            ["replicate"] = c => IngestSourceOutConfig.Replicate(ToObj<SvixConfigOut>(c)),
            ["resend"] = c => IngestSourceOutConfig.Resend(ToObj<SvixConfigOut>(c)),
            ["rutter"] = c => IngestSourceOutConfig.Rutter(ToObj<RutterConfigOut>(c)),
            ["safebase"] = c => IngestSourceOutConfig.Safebase(ToObj<SvixConfigOut>(c)),
            ["sardine"] = c => IngestSourceOutConfig.Sardine(ToObj<SvixConfigOut>(c)),
            ["segment"] = c => IngestSourceOutConfig.Segment(ToObj<SegmentConfigOut>(c)),
            ["shopify"] = c => IngestSourceOutConfig.Shopify(ToObj<ShopifyConfigOut>(c)),
            ["slack"] = c => IngestSourceOutConfig.Slack(ToObj<SlackConfigOut>(c)),
            ["stripe"] = c => IngestSourceOutConfig.Stripe(ToObj<StripeConfigOut>(c)),
            ["stych"] = c => IngestSourceOutConfig.Stych(ToObj<SvixConfigOut>(c)),
            ["svix"] = c => IngestSourceOutConfig.Svix(ToObj<SvixConfigOut>(c)),
            ["zoom"] = c => IngestSourceOutConfig.Zoom(ToObj<ZoomConfigOut>(c)),
            ["tailscale"] = c => IngestSourceOutConfig.Tailscale(ToObj<TailscaleConfigOut>(c)),
            ["telnyx"] = c => IngestSourceOutConfig.Telnyx(ToObj<TelnyxConfigOut>(c)),
            ["vapi"] = c => IngestSourceOutConfig.Vapi(ToObj<VapiConfigOut>(c)),
            ["open-ai"] = c => IngestSourceOutConfig.OpenAi(ToObj<SvixConfigOut>(c)),
            ["render"] = c => IngestSourceOutConfig.Render(ToObj<SvixConfigOut>(c)),
            ["veriff"] = c => IngestSourceOutConfig.Veriff(ToObj<VeriffConfigOut>(c)),
            ["airwallex"] = c => IngestSourceOutConfig.Airwallex(ToObj<AirwallexConfigOut>(c)),
            ["vgs"] = c => IngestSourceOutConfig.Vgs(ToObj<VgsConfigOut>(c)),
        };
    }
}
