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

        [EnumMember(Value = "Hubspot")]
        Hubspot = 7,

        [EnumMember(Value = "Inngest")]
        Inngest = 8,

        [EnumMember(Value = "Loops")]
        Loops = 9,

        [EnumMember(Value = "Otel")]
        Otel = 10,

        [EnumMember(Value = "Resend")]
        Resend = 11,

        [EnumMember(Value = "Salesforce")]
        Salesforce = 12,

        [EnumMember(Value = "Segment")]
        Segment = 13,

        [EnumMember(Value = "Sendgrid")]
        Sendgrid = 14,

        [EnumMember(Value = "Slack")]
        Slack = 15,

        [EnumMember(Value = "Teams")]
        Teams = 16,

        [EnumMember(Value = "TriggerDev")]
        TriggerDev = 17,

        [EnumMember(Value = "Windmill")]
        Windmill = 18,

        [EnumMember(Value = "Zapier")]
        Zapier = 19,
    }
}
