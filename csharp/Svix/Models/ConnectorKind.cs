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

        [EnumMember(Value = "CustomerIO")]
        CustomerIo = 2,

        [EnumMember(Value = "Discord")]
        Discord = 3,

        [EnumMember(Value = "Hubspot")]
        Hubspot = 4,

        [EnumMember(Value = "Inngest")]
        Inngest = 5,

        [EnumMember(Value = "Salesforce")]
        Salesforce = 6,

        [EnumMember(Value = "Segment")]
        Segment = 7,

        [EnumMember(Value = "Slack")]
        Slack = 8,

        [EnumMember(Value = "Teams")]
        Teams = 9,

        [EnumMember(Value = "TriggerDev")]
        TriggerDev = 10,

        [EnumMember(Value = "Windmill")]
        Windmill = 11,

        [EnumMember(Value = "Zapier")]
        Zapier = 12,
    }
}
