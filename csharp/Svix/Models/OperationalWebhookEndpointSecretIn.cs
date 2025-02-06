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
    /// OperationalWebhookEndpointSecretIn
    /// </summary>
    [DataContract(Name = "OperationalWebhookEndpointSecretIn")]
    public partial class OperationalWebhookEndpointSecretIn : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="OperationalWebhookEndpointSecretIn" /> class.
        /// </summary>
        /// <param name="key">The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret..</param>
        public OperationalWebhookEndpointSecretIn(string key = default(string))
        {
            this.Key = key;
        }

        /// <summary>
        /// The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret.
        /// </summary>
        /// <value>The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret.</value>
        /*
        <example>whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD</example>
        */
        [DataMember(Name = "key", EmitDefaultValue = true)]
        public string Key { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class OperationalWebhookEndpointSecretIn {\n");
            sb.Append("  Key: ").Append(Key).Append("\n");
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
            if (this.Key != null) {
                // Key (string) pattern
                Regex regexKey = new Regex(@"^(whsec_)?[a-zA-Z0-9+/=]{32,100}$", RegexOptions.CultureInvariant);
                if (!regexKey.Match(this.Key).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for Key, must match a pattern of " + regexKey, new [] { "Key" });
                }
            }

            yield break;
        }
    }

}
