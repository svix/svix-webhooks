// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(IngestSourceInConverter))]
    public class IngestSourceIn
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonIgnore]
        public required IngestSourceInConfig Config { get; set; }

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

            sb.Append("class IngestSourceIn {\n");
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class IngestSourceInConfig
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

        private IngestSourceInConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static IngestSourceInConfig GenericWebhook() =>
            new(new Dictionary<string, string>(), ConfigType.GenericWebhook);

        public static IngestSourceInConfig Cron(CronConfig cronConfig) =>
            new(cronConfig, ConfigType.Cron);

        public static IngestSourceInConfig AdobeSign(AdobeSignConfig adobeSignConfig) =>
            new(adobeSignConfig, ConfigType.AdobeSign);

        public static IngestSourceInConfig Beehiiv(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Beehiiv);

        public static IngestSourceInConfig Brex(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Brex);

        public static IngestSourceInConfig Checkbook(CheckbookConfig checkbookConfig) =>
            new(checkbookConfig, ConfigType.Checkbook);

        public static IngestSourceInConfig Clerk(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Clerk);

        public static IngestSourceInConfig Docusign(DocusignConfig docusignConfig) =>
            new(docusignConfig, ConfigType.Docusign);

        public static IngestSourceInConfig Easypost(EasypostConfig easypostConfig) =>
            new(easypostConfig, ConfigType.Easypost);

        public static IngestSourceInConfig Github(GithubConfig githubConfig) =>
            new(githubConfig, ConfigType.Github);

        public static IngestSourceInConfig Guesty(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Guesty);

        public static IngestSourceInConfig Hubspot(HubspotConfig hubspotConfig) =>
            new(hubspotConfig, ConfigType.Hubspot);

        public static IngestSourceInConfig IncidentIo(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.IncidentIo);

        public static IngestSourceInConfig Lithic(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Lithic);

        public static IngestSourceInConfig Nash(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Nash);

        public static IngestSourceInConfig OrumIo(OrumIoConfig orumIoConfig) =>
            new(orumIoConfig, ConfigType.OrumIo);

        public static IngestSourceInConfig PandaDoc(PandaDocConfig pandaDocConfig) =>
            new(pandaDocConfig, ConfigType.PandaDoc);

        public static IngestSourceInConfig PortIo(PortIoConfig portIoConfig) =>
            new(portIoConfig, ConfigType.PortIo);

        public static IngestSourceInConfig Pleo(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Pleo);

        public static IngestSourceInConfig Replicate(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Replicate);

        public static IngestSourceInConfig Resend(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Resend);

        public static IngestSourceInConfig Rutter(RutterConfig rutterConfig) =>
            new(rutterConfig, ConfigType.Rutter);

        public static IngestSourceInConfig Safebase(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Safebase);

        public static IngestSourceInConfig Sardine(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Sardine);

        public static IngestSourceInConfig Segment(SegmentConfig segmentConfig) =>
            new(segmentConfig, ConfigType.Segment);

        public static IngestSourceInConfig Shopify(ShopifyConfig shopifyConfig) =>
            new(shopifyConfig, ConfigType.Shopify);

        public static IngestSourceInConfig Slack(SlackConfig slackConfig) =>
            new(slackConfig, ConfigType.Slack);

        public static IngestSourceInConfig Stripe(StripeConfig stripeConfig) =>
            new(stripeConfig, ConfigType.Stripe);

        public static IngestSourceInConfig Stych(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Stych);

        public static IngestSourceInConfig Svix(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Svix);

        public static IngestSourceInConfig Zoom(ZoomConfig zoomConfig) =>
            new(zoomConfig, ConfigType.Zoom);

        public static IngestSourceInConfig Telnyx(TelnyxConfig telnyxConfig) =>
            new(telnyxConfig, ConfigType.Telnyx);

        public static IngestSourceInConfig Vapi(VapiConfig vapiConfig) =>
            new(vapiConfig, ConfigType.Vapi);

        public static IngestSourceInConfig OpenAi(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.OpenAi);

        public static IngestSourceInConfig Render(SvixConfig svixConfig) =>
            new(svixConfig, ConfigType.Render);

        public static IngestSourceInConfig Veriff(VeriffConfig veriffConfig) =>
            new(veriffConfig, ConfigType.Veriff);

        public static IngestSourceInConfig Airwallex(AirwallexConfig airwallexConfig) =>
            new(airwallexConfig, ConfigType.Airwallex);

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

            [EnumMember(Value = "nash")]
            Nash,

            [EnumMember(Value = "orum-io")]
            OrumIo,

            [EnumMember(Value = "panda-doc")]
            PandaDoc,

            [EnumMember(Value = "port-io")]
            PortIo,

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
        }

        public TResult Match<TResult>(
            Func<TResult> onGenericWebhook,
            Func<CronConfig, TResult> onCron,
            Func<AdobeSignConfig, TResult> onAdobeSign,
            Func<SvixConfig, TResult> onBeehiiv,
            Func<SvixConfig, TResult> onBrex,
            Func<CheckbookConfig, TResult> onCheckbook,
            Func<SvixConfig, TResult> onClerk,
            Func<DocusignConfig, TResult> onDocusign,
            Func<EasypostConfig, TResult> onEasypost,
            Func<GithubConfig, TResult> onGithub,
            Func<SvixConfig, TResult> onGuesty,
            Func<HubspotConfig, TResult> onHubspot,
            Func<SvixConfig, TResult> onIncidentIo,
            Func<SvixConfig, TResult> onLithic,
            Func<SvixConfig, TResult> onNash,
            Func<OrumIoConfig, TResult> onOrumIo,
            Func<PandaDocConfig, TResult> onPandaDoc,
            Func<PortIoConfig, TResult> onPortIo,
            Func<SvixConfig, TResult> onPleo,
            Func<SvixConfig, TResult> onReplicate,
            Func<SvixConfig, TResult> onResend,
            Func<RutterConfig, TResult> onRutter,
            Func<SvixConfig, TResult> onSafebase,
            Func<SvixConfig, TResult> onSardine,
            Func<SegmentConfig, TResult> onSegment,
            Func<ShopifyConfig, TResult> onShopify,
            Func<SlackConfig, TResult> onSlack,
            Func<StripeConfig, TResult> onStripe,
            Func<SvixConfig, TResult> onStych,
            Func<SvixConfig, TResult> onSvix,
            Func<ZoomConfig, TResult> onZoom,
            Func<TelnyxConfig, TResult> onTelnyx,
            Func<VapiConfig, TResult> onVapi,
            Func<SvixConfig, TResult> onOpenAi,
            Func<SvixConfig, TResult> onRender,
            Func<VeriffConfig, TResult> onVeriff,
            Func<AirwallexConfig, TResult> onAirwallex
        )
        {
            return _type switch
            {
                ConfigType.GenericWebhook => onGenericWebhook(),
                ConfigType.Cron => onCron((CronConfig)_value),
                ConfigType.AdobeSign => onAdobeSign((AdobeSignConfig)_value),
                ConfigType.Beehiiv => onBeehiiv((SvixConfig)_value),
                ConfigType.Brex => onBrex((SvixConfig)_value),
                ConfigType.Checkbook => onCheckbook((CheckbookConfig)_value),
                ConfigType.Clerk => onClerk((SvixConfig)_value),
                ConfigType.Docusign => onDocusign((DocusignConfig)_value),
                ConfigType.Easypost => onEasypost((EasypostConfig)_value),
                ConfigType.Github => onGithub((GithubConfig)_value),
                ConfigType.Guesty => onGuesty((SvixConfig)_value),
                ConfigType.Hubspot => onHubspot((HubspotConfig)_value),
                ConfigType.IncidentIo => onIncidentIo((SvixConfig)_value),
                ConfigType.Lithic => onLithic((SvixConfig)_value),
                ConfigType.Nash => onNash((SvixConfig)_value),
                ConfigType.OrumIo => onOrumIo((OrumIoConfig)_value),
                ConfigType.PandaDoc => onPandaDoc((PandaDocConfig)_value),
                ConfigType.PortIo => onPortIo((PortIoConfig)_value),
                ConfigType.Pleo => onPleo((SvixConfig)_value),
                ConfigType.Replicate => onReplicate((SvixConfig)_value),
                ConfigType.Resend => onResend((SvixConfig)_value),
                ConfigType.Rutter => onRutter((RutterConfig)_value),
                ConfigType.Safebase => onSafebase((SvixConfig)_value),
                ConfigType.Sardine => onSardine((SvixConfig)_value),
                ConfigType.Segment => onSegment((SegmentConfig)_value),
                ConfigType.Shopify => onShopify((ShopifyConfig)_value),
                ConfigType.Slack => onSlack((SlackConfig)_value),
                ConfigType.Stripe => onStripe((StripeConfig)_value),
                ConfigType.Stych => onStych((SvixConfig)_value),
                ConfigType.Svix => onSvix((SvixConfig)_value),
                ConfigType.Zoom => onZoom((ZoomConfig)_value),
                ConfigType.Telnyx => onTelnyx((TelnyxConfig)_value),
                ConfigType.Vapi => onVapi((VapiConfig)_value),
                ConfigType.OpenAi => onOpenAi((SvixConfig)_value),
                ConfigType.Render => onRender((SvixConfig)_value),
                ConfigType.Veriff => onVeriff((VeriffConfig)_value),
                ConfigType.Airwallex => onAirwallex((AirwallexConfig)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onGenericWebhook,
            Action<CronConfig> onCron,
            Action<AdobeSignConfig> onAdobeSign,
            Action<SvixConfig> onBeehiiv,
            Action<SvixConfig> onBrex,
            Action<CheckbookConfig> onCheckbook,
            Action<SvixConfig> onClerk,
            Action<DocusignConfig> onDocusign,
            Action<EasypostConfig> onEasypost,
            Action<GithubConfig> onGithub,
            Action<SvixConfig> onGuesty,
            Action<HubspotConfig> onHubspot,
            Action<SvixConfig> onIncidentIo,
            Action<SvixConfig> onLithic,
            Action<SvixConfig> onNash,
            Action<OrumIoConfig> onOrumIo,
            Action<PandaDocConfig> onPandaDoc,
            Action<PortIoConfig> onPortIo,
            Action<SvixConfig> onPleo,
            Action<SvixConfig> onReplicate,
            Action<SvixConfig> onResend,
            Action<RutterConfig> onRutter,
            Action<SvixConfig> onSafebase,
            Action<SvixConfig> onSardine,
            Action<SegmentConfig> onSegment,
            Action<ShopifyConfig> onShopify,
            Action<SlackConfig> onSlack,
            Action<StripeConfig> onStripe,
            Action<SvixConfig> onStych,
            Action<SvixConfig> onSvix,
            Action<ZoomConfig> onZoom,
            Action<TelnyxConfig> onTelnyx,
            Action<VapiConfig> onVapi,
            Action<SvixConfig> onOpenAi,
            Action<SvixConfig> onRender,
            Action<VeriffConfig> onVeriff,
            Action<AirwallexConfig> onAirwallex
        )
        {
            switch (_type)
            {
                case ConfigType.GenericWebhook:
                    onGenericWebhook();
                    break;
                case ConfigType.Cron:
                    onCron((CronConfig)_value);
                    break;
                case ConfigType.AdobeSign:
                    onAdobeSign((AdobeSignConfig)_value);
                    break;
                case ConfigType.Beehiiv:
                    onBeehiiv((SvixConfig)_value);
                    break;
                case ConfigType.Brex:
                    onBrex((SvixConfig)_value);
                    break;
                case ConfigType.Checkbook:
                    onCheckbook((CheckbookConfig)_value);
                    break;
                case ConfigType.Clerk:
                    onClerk((SvixConfig)_value);
                    break;
                case ConfigType.Docusign:
                    onDocusign((DocusignConfig)_value);
                    break;
                case ConfigType.Easypost:
                    onEasypost((EasypostConfig)_value);
                    break;
                case ConfigType.Github:
                    onGithub((GithubConfig)_value);
                    break;
                case ConfigType.Guesty:
                    onGuesty((SvixConfig)_value);
                    break;
                case ConfigType.Hubspot:
                    onHubspot((HubspotConfig)_value);
                    break;
                case ConfigType.IncidentIo:
                    onIncidentIo((SvixConfig)_value);
                    break;
                case ConfigType.Lithic:
                    onLithic((SvixConfig)_value);
                    break;
                case ConfigType.Nash:
                    onNash((SvixConfig)_value);
                    break;
                case ConfigType.OrumIo:
                    onOrumIo((OrumIoConfig)_value);
                    break;
                case ConfigType.PandaDoc:
                    onPandaDoc((PandaDocConfig)_value);
                    break;
                case ConfigType.PortIo:
                    onPortIo((PortIoConfig)_value);
                    break;
                case ConfigType.Pleo:
                    onPleo((SvixConfig)_value);
                    break;
                case ConfigType.Replicate:
                    onReplicate((SvixConfig)_value);
                    break;
                case ConfigType.Resend:
                    onResend((SvixConfig)_value);
                    break;
                case ConfigType.Rutter:
                    onRutter((RutterConfig)_value);
                    break;
                case ConfigType.Safebase:
                    onSafebase((SvixConfig)_value);
                    break;
                case ConfigType.Sardine:
                    onSardine((SvixConfig)_value);
                    break;
                case ConfigType.Segment:
                    onSegment((SegmentConfig)_value);
                    break;
                case ConfigType.Shopify:
                    onShopify((ShopifyConfig)_value);
                    break;
                case ConfigType.Slack:
                    onSlack((SlackConfig)_value);
                    break;
                case ConfigType.Stripe:
                    onStripe((StripeConfig)_value);
                    break;
                case ConfigType.Stych:
                    onStych((SvixConfig)_value);
                    break;
                case ConfigType.Svix:
                    onSvix((SvixConfig)_value);
                    break;
                case ConfigType.Zoom:
                    onZoom((ZoomConfig)_value);
                    break;
                case ConfigType.Telnyx:
                    onTelnyx((TelnyxConfig)_value);
                    break;
                case ConfigType.Vapi:
                    onVapi((VapiConfig)_value);
                    break;
                case ConfigType.OpenAi:
                    onOpenAi((SvixConfig)_value);
                    break;
                case ConfigType.Render:
                    onRender((SvixConfig)_value);
                    break;
                case ConfigType.Veriff:
                    onVeriff((VeriffConfig)_value);
                    break;
                case ConfigType.Airwallex:
                    onAirwallex((AirwallexConfig)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class IngestSourceInSurrogate
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public required JObject Config { get; set; }
    }

    public class IngestSourceInConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(IngestSourceIn);
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
                serializer.Deserialize<IngestSourceInSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to IngestSourceInSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), IngestSourceInConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for IngestSourceInConfig.config"
                );
            }

            IngestSourceInConfig config = func((surrogate.Config, surrogate.Type));

            return new IngestSourceIn
            {
                Metadata = surrogate.Metadata,
                Name = surrogate.Name,
                Uid = surrogate.Uid,
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

        private readonly Dictionary<string, Func<(JObject, string), IngestSourceInConfig>> typeMap =
            new()
            {
                ["generic-webhook"] = c => IngestSourceInConfig.GenericWebhook(),
                ["cron"] = c => IngestSourceInConfig.Cron(ToObj<CronConfig>(c)),
                ["adobe-sign"] = c => IngestSourceInConfig.AdobeSign(ToObj<AdobeSignConfig>(c)),
                ["beehiiv"] = c => IngestSourceInConfig.Beehiiv(ToObj<SvixConfig>(c)),
                ["brex"] = c => IngestSourceInConfig.Brex(ToObj<SvixConfig>(c)),
                ["checkbook"] = c => IngestSourceInConfig.Checkbook(ToObj<CheckbookConfig>(c)),
                ["clerk"] = c => IngestSourceInConfig.Clerk(ToObj<SvixConfig>(c)),
                ["docusign"] = c => IngestSourceInConfig.Docusign(ToObj<DocusignConfig>(c)),
                ["easypost"] = c => IngestSourceInConfig.Easypost(ToObj<EasypostConfig>(c)),
                ["github"] = c => IngestSourceInConfig.Github(ToObj<GithubConfig>(c)),
                ["guesty"] = c => IngestSourceInConfig.Guesty(ToObj<SvixConfig>(c)),
                ["hubspot"] = c => IngestSourceInConfig.Hubspot(ToObj<HubspotConfig>(c)),
                ["incident-io"] = c => IngestSourceInConfig.IncidentIo(ToObj<SvixConfig>(c)),
                ["lithic"] = c => IngestSourceInConfig.Lithic(ToObj<SvixConfig>(c)),
                ["nash"] = c => IngestSourceInConfig.Nash(ToObj<SvixConfig>(c)),
                ["orum-io"] = c => IngestSourceInConfig.OrumIo(ToObj<OrumIoConfig>(c)),
                ["panda-doc"] = c => IngestSourceInConfig.PandaDoc(ToObj<PandaDocConfig>(c)),
                ["port-io"] = c => IngestSourceInConfig.PortIo(ToObj<PortIoConfig>(c)),
                ["pleo"] = c => IngestSourceInConfig.Pleo(ToObj<SvixConfig>(c)),
                ["replicate"] = c => IngestSourceInConfig.Replicate(ToObj<SvixConfig>(c)),
                ["resend"] = c => IngestSourceInConfig.Resend(ToObj<SvixConfig>(c)),
                ["rutter"] = c => IngestSourceInConfig.Rutter(ToObj<RutterConfig>(c)),
                ["safebase"] = c => IngestSourceInConfig.Safebase(ToObj<SvixConfig>(c)),
                ["sardine"] = c => IngestSourceInConfig.Sardine(ToObj<SvixConfig>(c)),
                ["segment"] = c => IngestSourceInConfig.Segment(ToObj<SegmentConfig>(c)),
                ["shopify"] = c => IngestSourceInConfig.Shopify(ToObj<ShopifyConfig>(c)),
                ["slack"] = c => IngestSourceInConfig.Slack(ToObj<SlackConfig>(c)),
                ["stripe"] = c => IngestSourceInConfig.Stripe(ToObj<StripeConfig>(c)),
                ["stych"] = c => IngestSourceInConfig.Stych(ToObj<SvixConfig>(c)),
                ["svix"] = c => IngestSourceInConfig.Svix(ToObj<SvixConfig>(c)),
                ["zoom"] = c => IngestSourceInConfig.Zoom(ToObj<ZoomConfig>(c)),
                ["telnyx"] = c => IngestSourceInConfig.Telnyx(ToObj<TelnyxConfig>(c)),
                ["vapi"] = c => IngestSourceInConfig.Vapi(ToObj<VapiConfig>(c)),
                ["open-ai"] = c => IngestSourceInConfig.OpenAi(ToObj<SvixConfig>(c)),
                ["render"] = c => IngestSourceInConfig.Render(ToObj<SvixConfig>(c)),
                ["veriff"] = c => IngestSourceInConfig.Veriff(ToObj<VeriffConfig>(c)),
                ["airwallex"] = c => IngestSourceInConfig.Airwallex(ToObj<AirwallexConfig>(c)),
            };
    }
}
