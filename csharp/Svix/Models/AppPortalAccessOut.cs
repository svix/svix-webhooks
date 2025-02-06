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
    /// AppPortalAccessOut
    /// </summary>
    [DataContract(Name = "AppPortalAccessOut")]
    public partial class AppPortalAccessOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="AppPortalAccessOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected AppPortalAccessOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="AppPortalAccessOut" /> class.
        /// </summary>
        /// <param name="token">token (required).</param>
        /// <param name="url">url (required).</param>
        public AppPortalAccessOut(string token = default(string), string url = default(string))
        {
            // to ensure "token" is required (not null)
            if (token == null)
            {
                throw new ArgumentNullException("token is a required property for AppPortalAccessOut and cannot be null");
            }
            this.Token = token;
            // to ensure "url" is required (not null)
            if (url == null)
            {
                throw new ArgumentNullException("url is a required property for AppPortalAccessOut and cannot be null");
            }
            this.Url = url;
        }

        /// <summary>
        /// Gets or Sets Token
        /// </summary>
        /*
        <example>appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O</example>
        */
        [DataMember(Name = "token", IsRequired = true, EmitDefaultValue = true)]
        public string Token { get; set; }

        /// <summary>
        /// Gets or Sets Url
        /// </summary>
        /*
        <example>https://app.svix.com/login#key&#x3D;eyJhcHBJZCI6ICJhcHBfMXRSdFl</example>
        */
        [DataMember(Name = "url", IsRequired = true, EmitDefaultValue = true)]
        public string Url { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class AppPortalAccessOut {\n");
            sb.Append("  Token: ").Append(Token).Append("\n");
            sb.Append("  Url: ").Append(Url).Append("\n");
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
            // Url (string) maxLength
            if (this.Url != null && this.Url.Length > 65536)
            {
                yield return new ValidationResult("Invalid value for Url, length must be less than 65536.", new [] { "Url" });
            }

            // Url (string) minLength
            if (this.Url != null && this.Url.Length < 1)
            {
                yield return new ValidationResult("Invalid value for Url, length must be greater than 1.", new [] { "Url" });
            }

            yield break;
        }
    }

}
