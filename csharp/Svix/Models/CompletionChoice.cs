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
    /// CompletionChoice
    /// </summary>
    [DataContract(Name = "CompletionChoice")]
    public partial class CompletionChoice : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CompletionChoice" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CompletionChoice() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CompletionChoice" /> class.
        /// </summary>
        /// <param name="finishReason">finishReason (required).</param>
        /// <param name="index">index (required).</param>
        /// <param name="message">message (required).</param>
        public CompletionChoice(string finishReason = default(string), long index = default(long), CompletionMessage message = default(CompletionMessage))
        {
            // to ensure "finishReason" is required (not null)
            if (finishReason == null)
            {
                throw new ArgumentNullException("finishReason is a required property for CompletionChoice and cannot be null");
            }
            this.FinishReason = finishReason;
            this.Index = index;
            // to ensure "message" is required (not null)
            if (message == null)
            {
                throw new ArgumentNullException("message is a required property for CompletionChoice and cannot be null");
            }
            this.Message = message;
        }

        /// <summary>
        /// Gets or Sets FinishReason
        /// </summary>
        [DataMember(Name = "finish_reason", IsRequired = true, EmitDefaultValue = true)]
        public string FinishReason { get; set; }

        /// <summary>
        /// Gets or Sets Index
        /// </summary>
        [DataMember(Name = "index", IsRequired = true, EmitDefaultValue = true)]
        public long Index { get; set; }

        /// <summary>
        /// Gets or Sets Message
        /// </summary>
        [DataMember(Name = "message", IsRequired = true, EmitDefaultValue = true)]
        public CompletionMessage Message { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class CompletionChoice {\n");
            sb.Append("  FinishReason: ").Append(FinishReason).Append("\n");
            sb.Append("  Index: ").Append(Index).Append("\n");
            sb.Append("  Message: ").Append(Message).Append("\n");
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
