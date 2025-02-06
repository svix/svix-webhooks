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
    /// The value of the headers is returned in the &#x60;headers&#x60; field.  Sensitive headers that have been redacted are returned in the sensitive field.
    /// </summary>
    [DataContract(Name = "EndpointHeadersOut")]
    public partial class EndpointHeadersOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointHeadersOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EndpointHeadersOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointHeadersOut" /> class.
        /// </summary>
        /// <param name="headers">headers (required).</param>
        /// <param name="sensitive">sensitive (required).</param>
        public EndpointHeadersOut(Dictionary<string, string> headers = default(Dictionary<string, string>), List<string> sensitive = default(List<string>))
        {
            // to ensure "headers" is required (not null)
            if (headers == null)
            {
                throw new ArgumentNullException("headers is a required property for EndpointHeadersOut and cannot be null");
            }
            this.Headers = headers;
            // to ensure "sensitive" is required (not null)
            if (sensitive == null)
            {
                throw new ArgumentNullException("sensitive is a required property for EndpointHeadersOut and cannot be null");
            }
            this.Sensitive = sensitive;
        }

        /// <summary>
        /// Gets or Sets Headers
        /// </summary>
        /*
        <example>{&quot;X-Example&quot;:&quot;123&quot;,&quot;X-Foobar&quot;:&quot;Bar&quot;}</example>
        */
        [DataMember(Name = "headers", IsRequired = true, EmitDefaultValue = true)]
        public Dictionary<string, string> Headers { get; set; }

        /// <summary>
        /// Gets or Sets Sensitive
        /// </summary>
        /*
        <example>[&quot;Authorization&quot;]</example>
        */
        [DataMember(Name = "sensitive", IsRequired = true, EmitDefaultValue = true)]
        public List<string> Sensitive { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EndpointHeadersOut {\n");
            sb.Append("  Headers: ").Append(Headers).Append("\n");
            sb.Append("  Sensitive: ").Append(Sensitive).Append("\n");
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
