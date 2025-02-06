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
    /// EndpointOauthConfigOut
    /// </summary>
    [DataContract(Name = "EndpointOauthConfigOut")]
    public partial class EndpointOauthConfigOut : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets AuthMethod
        /// </summary>
        [DataMember(Name = "authMethod", IsRequired = true, EmitDefaultValue = true)]
        public Oauth2AuthMethodInOut AuthMethod { get; set; }

        /// <summary>
        /// Gets or Sets GrantType
        /// </summary>
        [DataMember(Name = "grantType", IsRequired = true, EmitDefaultValue = true)]
        public Oauth2GrantTypeInOut GrantType { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointOauthConfigOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EndpointOauthConfigOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointOauthConfigOut" /> class.
        /// </summary>
        /// <param name="authMethod">authMethod (required).</param>
        /// <param name="clientId">clientId (required).</param>
        /// <param name="extraParams">extraParams.</param>
        /// <param name="grantType">grantType (required).</param>
        /// <param name="scopes">scopes.</param>
        /// <param name="tokenUrl">tokenUrl (required).</param>
        public EndpointOauthConfigOut(Oauth2AuthMethodInOut authMethod = default(Oauth2AuthMethodInOut), string clientId = default(string), Dictionary<string, string> extraParams = default(Dictionary<string, string>), Oauth2GrantTypeInOut grantType = default(Oauth2GrantTypeInOut), List<string> scopes = default(List<string>), string tokenUrl = default(string))
        {
            this.AuthMethod = authMethod;
            // to ensure "clientId" is required (not null)
            if (clientId == null)
            {
                throw new ArgumentNullException("clientId is a required property for EndpointOauthConfigOut and cannot be null");
            }
            this.ClientId = clientId;
            this.GrantType = grantType;
            // to ensure "tokenUrl" is required (not null)
            if (tokenUrl == null)
            {
                throw new ArgumentNullException("tokenUrl is a required property for EndpointOauthConfigOut and cannot be null");
            }
            this.TokenUrl = tokenUrl;
            this.ExtraParams = extraParams;
            this.Scopes = scopes;
        }

        /// <summary>
        /// Gets or Sets ClientId
        /// </summary>
        [DataMember(Name = "clientId", IsRequired = true, EmitDefaultValue = true)]
        public string ClientId { get; set; }

        /// <summary>
        /// Gets or Sets ExtraParams
        /// </summary>
        [DataMember(Name = "extraParams", EmitDefaultValue = true)]
        public Dictionary<string, string> ExtraParams { get; set; }

        /// <summary>
        /// Gets or Sets Scopes
        /// </summary>
        [DataMember(Name = "scopes", EmitDefaultValue = true)]
        public List<string> Scopes { get; set; }

        /// <summary>
        /// Gets or Sets TokenUrl
        /// </summary>
        [DataMember(Name = "tokenUrl", IsRequired = true, EmitDefaultValue = true)]
        public string TokenUrl { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EndpointOauthConfigOut {\n");
            sb.Append("  AuthMethod: ").Append(AuthMethod).Append("\n");
            sb.Append("  ClientId: ").Append(ClientId).Append("\n");
            sb.Append("  ExtraParams: ").Append(ExtraParams).Append("\n");
            sb.Append("  GrantType: ").Append(GrantType).Append("\n");
            sb.Append("  Scopes: ").Append(Scopes).Append("\n");
            sb.Append("  TokenUrl: ").Append(TokenUrl).Append("\n");
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
