/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using FileParameter = Svix.Client.FileParameter;
using OpenAPIDateConverter = Svix.Client.OpenAPIDateConverter;

namespace Svix.Model
{
    /// <summary>
    /// MessageOut
    /// </summary>
    [DataContract(Name = "MessageOut")]
    public partial class MessageOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="MessageOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected MessageOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="MessageOut" /> class.
        /// </summary>
        /// <param name="channels">List of free-form identifiers that endpoints can filter by.</param>
        /// <param name="eventId">Optional unique identifier for the message.</param>
        /// <param name="eventType">The event type&#39;s name (required).</param>
        /// <param name="id">The msg&#39;s ID (required).</param>
        /// <param name="payload">payload (required).</param>
        /// <param name="tags">tags.</param>
        /// <param name="timestamp">timestamp (required).</param>
        public MessageOut(List<string> channels = default(List<string>), string eventId = default(string), string eventType = default(string), string id = default(string), Object payload = default(Object), List<string> tags = default(List<string>), DateTime timestamp = default(DateTime))
        {
            // to ensure "eventType" is required (not null)
            if (eventType == null)
            {
                throw new ArgumentNullException("eventType is a required property for MessageOut and cannot be null");
            }
            this.EventType = eventType;
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for MessageOut and cannot be null");
            }
            this.Id = id;
            // to ensure "payload" is required (not null)
            if (payload == null)
            {
                throw new ArgumentNullException("payload is a required property for MessageOut and cannot be null");
            }
            this.Payload = payload;
            this.Timestamp = timestamp;
            this.Channels = channels;
            this.EventId = eventId;
            this.Tags = tags;
        }

        /// <summary>
        /// List of free-form identifiers that endpoints can filter by
        /// </summary>
        /// <value>List of free-form identifiers that endpoints can filter by</value>
        /*
        <example>[&quot;project_123&quot;,&quot;group_2&quot;]</example>
        */
        [DataMember(Name = "channels", EmitDefaultValue = true)]
        public List<string> Channels { get; set; }

        /// <summary>
        /// Optional unique identifier for the message
        /// </summary>
        /// <value>Optional unique identifier for the message</value>
        /*
        <example>unique-msg-identifier</example>
        */
        [DataMember(Name = "eventId", EmitDefaultValue = true)]
        public string EventId { get; set; }

        /// <summary>
        /// The event type&#39;s name
        /// </summary>
        /// <value>The event type&#39;s name</value>
        /*
        <example>user.signup</example>
        */
        [DataMember(Name = "eventType", IsRequired = true, EmitDefaultValue = true)]
        public string EventType { get; set; }

        /// <summary>
        /// The msg&#39;s ID
        /// </summary>
        /// <value>The msg&#39;s ID</value>
        /*
        <example>msg_1srOrx2ZWZBpBUvZwXKQmoEYga2</example>
        */
        [DataMember(Name = "id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// Gets or Sets Payload
        /// </summary>
        /*
        <example>{&quot;email&quot;:&quot;test@example.com&quot;,&quot;type&quot;:&quot;user.created&quot;,&quot;username&quot;:&quot;test_user&quot;}</example>
        */
        [DataMember(Name = "payload", IsRequired = true, EmitDefaultValue = true)]
        public Object Payload { get; set; }

        /// <summary>
        /// Gets or Sets Tags
        /// </summary>
        [DataMember(Name = "tags", EmitDefaultValue = true)]
        public List<string> Tags { get; set; }

        /// <summary>
        /// Gets or Sets Timestamp
        /// </summary>
        [DataMember(Name = "timestamp", IsRequired = true, EmitDefaultValue = true)]
        public DateTime Timestamp { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class MessageOut {\n");
            sb.Append("  Channels: ").Append(Channels).Append("\n");
            sb.Append("  EventId: ").Append(EventId).Append("\n");
            sb.Append("  EventType: ").Append(EventType).Append("\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  Payload: ").Append(Payload).Append("\n");
            sb.Append("  Tags: ").Append(Tags).Append("\n");
            sb.Append("  Timestamp: ").Append(Timestamp).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            // EventId (string) maxLength
            if (this.EventId != null && this.EventId.Length > 256)
            {
                yield return new ValidationResult("Invalid value for EventId, length must be less than 256.", new [] { "EventId" });
            }

            // EventId (string) minLength
            if (this.EventId != null && this.EventId.Length < 1)
            {
                yield return new ValidationResult("Invalid value for EventId, length must be greater than 1.", new [] { "EventId" });
            }

            if (this.EventId != null) {
                // EventId (string) pattern
                Regex regexEventId = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexEventId.Match(this.EventId).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for EventId, must match a pattern of " + regexEventId, new [] { "EventId" });
                }
            }

            // EventType (string) maxLength
            if (this.EventType != null && this.EventType.Length > 256)
            {
                yield return new ValidationResult("Invalid value for EventType, length must be less than 256.", new [] { "EventType" });
            }

            if (this.EventType != null) {
                // EventType (string) pattern
                Regex regexEventType = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexEventType.Match(this.EventType).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for EventType, must match a pattern of " + regexEventType, new [] { "EventType" });
                }
            }

            yield break;
        }
    }

}
