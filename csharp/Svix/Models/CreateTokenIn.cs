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
    /// CreateTokenIn
    /// </summary>
    [DataContract(Name = "CreateTokenIn")]
    public partial class CreateTokenIn : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CreateTokenIn" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CreateTokenIn() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CreateTokenIn" /> class.
        /// </summary>
        /// <param name="expiry">How long the token will be valid for, in seconds..</param>
        /// <param name="name">The name of the token. (required).</param>
        public CreateTokenIn(int? expiry = default(int?), string name = default(string))
        {
            // to ensure "name" is required (not null)
            if (name == null)
            {
                throw new ArgumentNullException("name is a required property for CreateTokenIn and cannot be null");
            }
            this.Name = name;
            this.Expiry = expiry;
        }

        /// <summary>
        /// How long the token will be valid for, in seconds.
        /// </summary>
        /// <value>How long the token will be valid for, in seconds.</value>
        [DataMember(Name = "expiry", EmitDefaultValue = true)]
        public int? Expiry { get; set; }

        /// <summary>
        /// The name of the token.
        /// </summary>
        /// <value>The name of the token.</value>
        [DataMember(Name = "name", IsRequired = true, EmitDefaultValue = true)]
        public string Name { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class CreateTokenIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append("\n");
            sb.Append("  Name: ").Append(Name).Append("\n");
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
            // Expiry (int?) minimum
            if (this.Expiry < (int?)0)
            {
                yield return new ValidationResult("Invalid value for Expiry, must be a value greater than or equal to 0.", new [] { "Expiry" });
            }

            yield break;
        }
    }

}
