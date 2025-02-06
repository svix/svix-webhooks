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
    /// DataAnyOf2
    /// </summary>
    [DataContract(Name = "Data_anyOf_2")]
    public partial class DataAnyOf2 : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="DataAnyOf2" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected DataAnyOf2() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="DataAnyOf2" /> class.
        /// </summary>
        /// <param name="messagesCreated">messagesCreated (required).</param>
        /// <param name="messagesFailed">messagesFailed (required).</param>
        public DataAnyOf2(List<ApplicationMessageIdPair> messagesCreated = default(List<ApplicationMessageIdPair>), List<ApplicationMessageFailure> messagesFailed = default(List<ApplicationMessageFailure>))
        {
            // to ensure "messagesCreated" is required (not null)
            if (messagesCreated == null)
            {
                throw new ArgumentNullException("messagesCreated is a required property for DataAnyOf2 and cannot be null");
            }
            this.MessagesCreated = messagesCreated;
            // to ensure "messagesFailed" is required (not null)
            if (messagesFailed == null)
            {
                throw new ArgumentNullException("messagesFailed is a required property for DataAnyOf2 and cannot be null");
            }
            this.MessagesFailed = messagesFailed;
        }

        /// <summary>
        /// Gets or Sets MessagesCreated
        /// </summary>
        [DataMember(Name = "messages_created", IsRequired = true, EmitDefaultValue = true)]
        public List<ApplicationMessageIdPair> MessagesCreated { get; set; }

        /// <summary>
        /// Gets or Sets MessagesFailed
        /// </summary>
        [DataMember(Name = "messages_failed", IsRequired = true, EmitDefaultValue = true)]
        public List<ApplicationMessageFailure> MessagesFailed { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class DataAnyOf2 {\n");
            sb.Append("  MessagesCreated: ").Append(MessagesCreated).Append("\n");
            sb.Append("  MessagesFailed: ").Append(MessagesFailed).Append("\n");
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
            yield break;
        }
    }

}
