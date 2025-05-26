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

        [EnumMember(Value = "CloseCRM")]
        CloseCrm = 2,

        [EnumMember(Value = "CustomerIO")]
        CustomerIo = 3,

        [EnumMember(Value = "Discord")]
        Discord = 4,

        [EnumMember(Value = "Hubspot")]
        Hubspot = 5,

        [EnumMember(Value = "Inngest")]
        Inngest = 6,

        [EnumMember(Value = "Loops")]
        Loops = 7,

        [EnumMember(Value = "Resend")]
        Resend = 8,

        [EnumMember(Value = "Salesforce")]
        Salesforce = 9,

        [EnumMember(Value = "Segment")]
        Segment = 10,

        [EnumMember(Value = "Sendgrid")]
        Sendgrid = 11,

        [EnumMember(Value = "Slack")]
        Slack = 12,

        [EnumMember(Value = "Teams")]
        Teams = 13,

        [EnumMember(Value = "TriggerDev")]
        TriggerDev = 14,

        [EnumMember(Value = "Windmill")]
        Windmill = 15,

        [EnumMember(Value = "Zapier")]
        Zapier = 16,
    }
}
