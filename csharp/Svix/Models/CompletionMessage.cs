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
    /// CompletionMessage
    /// </summary>
    [DataContract(Name = "CompletionMessage")]
    public partial class CompletionMessage : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CompletionMessage" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CompletionMessage() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CompletionMessage" /> class.
        /// </summary>
        /// <param name="content">content (required).</param>
        /// <param name="role">role (required).</param>
        public CompletionMessage(string content = default(string), string role = default(string))
        {
            // to ensure "content" is required (not null)
            if (content == null)
            {
                throw new ArgumentNullException("content is a required property for CompletionMessage and cannot be null");
            }
            this.Content = content;
            // to ensure "role" is required (not null)
            if (role == null)
            {
                throw new ArgumentNullException("role is a required property for CompletionMessage and cannot be null");
            }
            this.Role = role;
        }

        /// <summary>
        /// Gets or Sets Content
        /// </summary>
        [DataMember(Name = "content", IsRequired = true, EmitDefaultValue = true)]
        public string Content { get; set; }

        /// <summary>
        /// Gets or Sets Role
        /// </summary>
        [DataMember(Name = "role", IsRequired = true, EmitDefaultValue = true)]
        public string Role { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class CompletionMessage {\n");
            sb.Append("  Content: ").Append(Content).Append("\n");
            sb.Append("  Role: ").Append(Role).Append("\n");
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
