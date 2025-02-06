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
    /// EndpointIn
    /// </summary>
    [DataContract(Name = "EndpointIn")]
    public partial class EndpointIn : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointIn" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EndpointIn() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EndpointIn" /> class.
        /// </summary>
        /// <param name="channels">List of message channels this endpoint listens to (omit for all)..</param>
        /// <param name="description">description (default to &quot;&quot;).</param>
        /// <param name="disabled">disabled (default to false).</param>
        /// <param name="filterTypes">filterTypes.</param>
        /// <param name="metadata">metadata.</param>
        /// <param name="rateLimit">rateLimit.</param>
        /// <param name="secret">The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret..</param>
        /// <param name="uid">Optional unique identifier for the endpoint..</param>
        /// <param name="url">url (required).</param>
        /// <param name="varVersion">varVersion (default to 1).</param>
        public EndpointIn(List<string> channels = default(List<string>), string description = @"", bool disabled = false, List<string> filterTypes = default(List<string>), Dictionary<string, string> metadata = default(Dictionary<string, string>), int? rateLimit = default(int?), string secret = default(string), string uid = default(string), string url = default(string), int? varVersion = 1)
        {
            // to ensure "url" is required (not null)
            if (url == null)
            {
                throw new ArgumentNullException("url is a required property for EndpointIn and cannot be null");
            }
            this.Url = url;
            this.Channels = channels;
            // use default value if no "description" provided
            this.Description = description ?? @"";
            this.Disabled = disabled;
            this.FilterTypes = filterTypes;
            this.Metadata = metadata;
            this.RateLimit = rateLimit;
            this.Secret = secret;
            this.Uid = uid;
            // use default value if no "varVersion" provided
            this.VarVersion = varVersion ?? 1;
        }

        /// <summary>
        /// List of message channels this endpoint listens to (omit for all).
        /// </summary>
        /// <value>List of message channels this endpoint listens to (omit for all).</value>
        /*
        <example>[&quot;project_123&quot;,&quot;group_2&quot;]</example>
        */
        [DataMember(Name = "channels", EmitDefaultValue = true)]
        public List<string> Channels { get; set; }

        /// <summary>
        /// Gets or Sets Description
        /// </summary>
        /*
        <example>An example endpoint name</example>
        */
        [DataMember(Name = "description", EmitDefaultValue = false)]
        public string Description { get; set; }

        /// <summary>
        /// Gets or Sets Disabled
        /// </summary>
        /*
        <example>false</example>
        */
        [DataMember(Name = "disabled", EmitDefaultValue = true)]
        public bool Disabled { get; set; }

        /// <summary>
        /// Gets or Sets FilterTypes
        /// </summary>
        /*
        <example>[&quot;user.signup&quot;,&quot;user.deleted&quot;]</example>
        */
        [DataMember(Name = "filterTypes", EmitDefaultValue = true)]
        public List<string> FilterTypes { get; set; }

        /// <summary>
        /// Gets or Sets Metadata
        /// </summary>
        [DataMember(Name = "metadata", EmitDefaultValue = false)]
        public Dictionary<string, string> Metadata { get; set; }

        /// <summary>
        /// Gets or Sets RateLimit
        /// </summary>
        [DataMember(Name = "rateLimit", EmitDefaultValue = true)]
        public int? RateLimit { get; set; }

        /// <summary>
        /// The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret.
        /// </summary>
        /// <value>The endpoint&#39;s verification secret.  Format: &#x60;base64&#x60; encoded random bytes optionally prefixed with &#x60;whsec_&#x60;. It is recommended to not set this and let the server generate the secret.</value>
        /*
        <example>whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD</example>
        */
        [DataMember(Name = "secret", EmitDefaultValue = true)]
        public string Secret { get; set; }

        /// <summary>
        /// Optional unique identifier for the endpoint.
        /// </summary>
        /// <value>Optional unique identifier for the endpoint.</value>
        /*
        <example>unique-ep-identifier</example>
        */
        [DataMember(Name = "uid", EmitDefaultValue = true)]
        public string Uid { get; set; }

        /// <summary>
        /// Gets or Sets Url
        /// </summary>
        /*
        <example>https://example.com/webhook/</example>
        */
        [DataMember(Name = "url", IsRequired = true, EmitDefaultValue = true)]
        public string Url { get; set; }

        /// <summary>
        /// Gets or Sets VarVersion
        /// </summary>
        /*
        <example>1</example>
        */
        [DataMember(Name = "version", EmitDefaultValue = true)]
        [Obsolete]
        public int? VarVersion { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EndpointIn {\n");
            sb.Append("  Channels: ").Append(Channels).Append("\n");
            sb.Append("  Description: ").Append(Description).Append("\n");
            sb.Append("  Disabled: ").Append(Disabled).Append("\n");
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append("\n");
            sb.Append("  Metadata: ").Append(Metadata).Append("\n");
            sb.Append("  RateLimit: ").Append(RateLimit).Append("\n");
            sb.Append("  Secret: ").Append(Secret).Append("\n");
            sb.Append("  Uid: ").Append(Uid).Append("\n");
            sb.Append("  Url: ").Append(Url).Append("\n");
            sb.Append("  VarVersion: ").Append(VarVersion).Append("\n");
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
            // RateLimit (int?) minimum
            if (this.RateLimit < (int?)1)
            {
                yield return new ValidationResult("Invalid value for RateLimit, must be a value greater than or equal to 1.", new [] { "RateLimit" });
            }

            if (this.Secret != null) {
                // Secret (string) pattern
                Regex regexSecret = new Regex(@"^(whsec_)?[a-zA-Z0-9+/=]{32,100}$", RegexOptions.CultureInvariant);
                if (!regexSecret.Match(this.Secret).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for Secret, must match a pattern of " + regexSecret, new [] { "Secret" });
                }
            }

            // Uid (string) maxLength
            if (this.Uid != null && this.Uid.Length > 256)
            {
                yield return new ValidationResult("Invalid value for Uid, length must be less than 256.", new [] { "Uid" });
            }

            // Uid (string) minLength
            if (this.Uid != null && this.Uid.Length < 1)
            {
                yield return new ValidationResult("Invalid value for Uid, length must be greater than 1.", new [] { "Uid" });
            }

            if (this.Uid != null) {
                // Uid (string) pattern
                Regex regexUid = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexUid.Match(this.Uid).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for Uid, must match a pattern of " + regexUid, new [] { "Uid" });
                }
            }

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

            // VarVersion (int?) minimum
            if (this.VarVersion < (int?)1)
            {
                yield return new ValidationResult("Invalid value for VarVersion, must be a value greater than or equal to 1.", new [] { "VarVersion" });
            }

            yield break;
        }
    }

}
