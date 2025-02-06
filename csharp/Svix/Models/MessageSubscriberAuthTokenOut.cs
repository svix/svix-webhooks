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
    /// MessageSubscriberAuthTokenOut
    /// </summary>
    [DataContract(Name = "MessageSubscriberAuthTokenOut")]
    public partial class MessageSubscriberAuthTokenOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="MessageSubscriberAuthTokenOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected MessageSubscriberAuthTokenOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="MessageSubscriberAuthTokenOut" /> class.
        /// </summary>
        /// <param name="bridgeToken">bridgeToken (required).</param>
        /// <param name="token">token (required).</param>
        public MessageSubscriberAuthTokenOut(string bridgeToken = default(string), string token = default(string))
        {
            // to ensure "bridgeToken" is required (not null)
            if (bridgeToken == null)
            {
                throw new ArgumentNullException("bridgeToken is a required property for MessageSubscriberAuthTokenOut and cannot be null");
            }
            this.BridgeToken = bridgeToken;
            // to ensure "token" is required (not null)
            if (token == null)
            {
                throw new ArgumentNullException("token is a required property for MessageSubscriberAuthTokenOut and cannot be null");
            }
            this.Token = token;
        }

        /// <summary>
        /// Gets or Sets BridgeToken
        /// </summary>
        [DataMember(Name = "bridgeToken", IsRequired = true, EmitDefaultValue = true)]
        public string BridgeToken { get; set; }

        /// <summary>
        /// Gets or Sets Token
        /// </summary>
        [DataMember(Name = "token", IsRequired = true, EmitDefaultValue = true)]
        public string Token { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class MessageSubscriberAuthTokenOut {\n");
            sb.Append("  BridgeToken: ").Append(BridgeToken).Append("\n");
            sb.Append("  Token: ").Append(Token).Append("\n");
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
