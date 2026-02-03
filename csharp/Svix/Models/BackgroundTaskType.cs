// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
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

        [EnumMember(Value = "application.purge_content")]
        ApplicationPurgeContent = 7,

        [EnumMember(Value = "endpoint.bulk-replay")]
        EndpointBulkReplay = 8,
    }
}
