// this file is @generated
using System.Runtime.Serialization;
using System.Text.Json.Serialization;

namespace Svix.Models
{
    [JsonConverter(typeof(JsonEnumMemberStringEnumConverter))]
    public enum BackgroundTaskType
    {
        [EnumMember(Value = "endpoint.replay")]
        EndpointReplay = 1,

        [EnumMember(Value = "endpoint.recover")]
        EndpointRecover = 2,

        [EnumMember(Value = "application.stats")]
        ApplicationStats = 3,

        [EnumMember(Value = "message.broadcast")]
        MessageBroadcast = 4,

        [EnumMember(Value = "sdk.generate")]
        SdkGenerate = 5,

        [EnumMember(Value = "event-type.aggregate")]
        EventTypeAggregate = 6,
    }
}
