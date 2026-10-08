// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum ConnectorKind
    {
        [EnumMember(Value = "Custom")]
        Custom = 1,

        [EnumMember(Value = "AgenticCommerceProtocol")]
        AgenticCommerceProtocol = 2,

        [EnumMember(Value = "CloseCRM")]
        CloseCrm = 3,

        [EnumMember(Value = "CustomerIO")]
        CustomerIo = 4,

        [EnumMember(Value = "DatadogTracing")]
        DatadogTracing = 5,

        [EnumMember(Value = "Discord")]
        Discord = 6,

        [EnumMember(Value = "GrafanaCloudTracing")]
        GrafanaCloudTracing = 7,

        [EnumMember(Value = "Hubspot")]
        Hubspot = 8,

        [EnumMember(Value = "Inngest")]
        Inngest = 9,

        [EnumMember(Value = "Loops")]
        Loops = 10,

        [EnumMember(Value = "NewRelicTracing")]
        NewRelicTracing = 11,

        [EnumMember(Value = "Otel")]
        Otel = 12,

        [EnumMember(Value = "Resend")]
        Resend = 13,

        [EnumMember(Value = "Salesforce")]
        Salesforce = 14,

        [EnumMember(Value = "Segment")]
        Segment = 15,

        [EnumMember(Value = "Sendgrid")]
        Sendgrid = 16,

        [EnumMember(Value = "Slack")]
        Slack = 17,

        [EnumMember(Value = "Teams")]
        Teams = 18,

        [EnumMember(Value = "TriggerDev")]
        TriggerDev = 19,

        [EnumMember(Value = "Windmill")]
        Windmill = 20,

        [EnumMember(Value = "Zapier")]
        Zapier = 21,
    }
}
