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
    /// SinkInOneOf1
    /// </summary>
    [DataContract(Name = "SinkIn_oneOf_1")]
    public partial class SinkInOneOf1 : IValidatableObject
    {
        /// <summary>
        /// Defines Type
        /// </summary>
        [JsonConverter(typeof(StringEnumConverter))]
        public enum TypeEnum
        {
            /// <summary>
            /// Enum Sqs for value: sqs
            /// </summary>
            [EnumMember(Value = "sqs")]
            Sqs = 1
        }


        /// <summary>
        /// Gets or Sets Type
        /// </summary>
        [DataMember(Name = "type", IsRequired = true, EmitDefaultValue = true)]
        public TypeEnum Type { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="SinkInOneOf1" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected SinkInOneOf1() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="SinkInOneOf1" /> class.
        /// </summary>
        /// <param name="accessKey">accessKey (required).</param>
        /// <param name="queueDsn">queueDsn (required).</param>
        /// <param name="region">region (required).</param>
        /// <param name="secretKey">secretKey (required).</param>
        /// <param name="type">type (required).</param>
        public SinkInOneOf1(string accessKey = default(string), string queueDsn = default(string), string region = default(string), string secretKey = default(string), TypeEnum type = default(TypeEnum))
        {
            // to ensure "accessKey" is required (not null)
            if (accessKey == null)
            {
                throw new ArgumentNullException("accessKey is a required property for SinkInOneOf1 and cannot be null");
            }
            this.AccessKey = accessKey;
            // to ensure "queueDsn" is required (not null)
            if (queueDsn == null)
            {
                throw new ArgumentNullException("queueDsn is a required property for SinkInOneOf1 and cannot be null");
            }
            this.QueueDsn = queueDsn;
            // to ensure "region" is required (not null)
            if (region == null)
            {
                throw new ArgumentNullException("region is a required property for SinkInOneOf1 and cannot be null");
            }
            this.Region = region;
            // to ensure "secretKey" is required (not null)
            if (secretKey == null)
            {
                throw new ArgumentNullException("secretKey is a required property for SinkInOneOf1 and cannot be null");
            }
            this.SecretKey = secretKey;
            this.Type = type;
        }

        /// <summary>
        /// Gets or Sets AccessKey
        /// </summary>
        [DataMember(Name = "accessKey", IsRequired = true, EmitDefaultValue = true)]
        public string AccessKey { get; set; }

        /// <summary>
        /// Gets or Sets QueueDsn
        /// </summary>
        [DataMember(Name = "queueDsn", IsRequired = true, EmitDefaultValue = true)]
        public string QueueDsn { get; set; }

        /// <summary>
        /// Gets or Sets Region
        /// </summary>
        [DataMember(Name = "region", IsRequired = true, EmitDefaultValue = true)]
        public string Region { get; set; }

        /// <summary>
        /// Gets or Sets SecretKey
        /// </summary>
        [DataMember(Name = "secretKey", IsRequired = true, EmitDefaultValue = true)]
        public string SecretKey { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class SinkInOneOf1 {\n");
            sb.Append("  AccessKey: ").Append(AccessKey).Append("\n");
            sb.Append("  QueueDsn: ").Append(QueueDsn).Append("\n");
            sb.Append("  Region: ").Append(Region).Append("\n");
            sb.Append("  SecretKey: ").Append(SecretKey).Append("\n");
            sb.Append("  Type: ").Append(Type).Append("\n");
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
