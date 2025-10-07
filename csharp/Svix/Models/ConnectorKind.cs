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

        [EnumMember(Value = "Discord")]
        Discord = 5,

        [EnumMember(Value = "Hubspot")]
        Hubspot = 6,

        [EnumMember(Value = "Inngest")]
        Inngest = 7,

        [EnumMember(Value = "Loops")]
        Loops = 8,

        [EnumMember(Value = "Resend")]
        Resend = 9,

        [EnumMember(Value = "Salesforce")]
        Salesforce = 10,

        [EnumMember(Value = "Segment")]
        Segment = 11,

        [EnumMember(Value = "Sendgrid")]
        Sendgrid = 12,

        [EnumMember(Value = "Slack")]
        Slack = 13,

        [EnumMember(Value = "Teams")]
        Teams = 14,

        [EnumMember(Value = "TriggerDev")]
        TriggerDev = 15,

        [EnumMember(Value = "Windmill")]
        Windmill = 16,

        [EnumMember(Value = "Zapier")]
        Zapier = 17,
    }
}
