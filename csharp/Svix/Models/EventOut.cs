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
    /// EventOut
    /// </summary>
    [DataContract(Name = "EventOut")]
    public partial class EventOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EventOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EventOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EventOut" /> class.
        /// </summary>
        /// <param name="eventType">The event type&#39;s name.</param>
        /// <param name="payload">payload (required).</param>
        /// <param name="timestamp">timestamp (required).</param>
        public EventOut(string eventType = default(string), string payload = default(string), DateTime timestamp = default(DateTime))
        {
            // to ensure "payload" is required (not null)
            if (payload == null)
            {
                throw new ArgumentNullException("payload is a required property for EventOut and cannot be null");
            }
            this.Payload = payload;
            this.Timestamp = timestamp;
            this.EventType = eventType;
        }

        /// <summary>
        /// The event type&#39;s name
        /// </summary>
        /// <value>The event type&#39;s name</value>
        /*
        <example>user.signup</example>
        */
        [DataMember(Name = "eventType", EmitDefaultValue = true)]
        public string EventType { get; set; }

        /// <summary>
        /// Gets or Sets Payload
        /// </summary>
        [DataMember(Name = "payload", IsRequired = true, EmitDefaultValue = true)]
        public string Payload { get; set; }

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
            sb.Append("class EventOut {\n");
            sb.Append("  EventType: ").Append(EventType).Append("\n");
            sb.Append("  Payload: ").Append(Payload).Append("\n");
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
