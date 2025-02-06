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
    /// EventTypeOut
    /// </summary>
    [DataContract(Name = "EventTypeOut")]
    public partial class EventTypeOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EventTypeOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EventTypeOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EventTypeOut" /> class.
        /// </summary>
        /// <param name="archived">archived (default to false).</param>
        /// <param name="createdAt">createdAt (required).</param>
        /// <param name="deprecated">deprecated (required).</param>
        /// <param name="description">description (required).</param>
        /// <param name="featureFlag">featureFlag.</param>
        /// <param name="groupName">The event type group&#39;s name.</param>
        /// <param name="name">The event type&#39;s name (required).</param>
        /// <param name="schemas">The schema for the event type for a specific version as a JSON schema..</param>
        /// <param name="updatedAt">updatedAt (required).</param>
        public EventTypeOut(bool archived = false, DateTime createdAt = default(DateTime), bool deprecated = default(bool), string description = default(string), string featureFlag = default(string), string groupName = default(string), string name = default(string), Object schemas = default(Object), DateTime updatedAt = default(DateTime))
        {
            this.CreatedAt = createdAt;
            this.Deprecated = deprecated;
            // to ensure "description" is required (not null)
            if (description == null)
            {
                throw new ArgumentNullException("description is a required property for EventTypeOut and cannot be null");
            }
            this.Description = description;
            // to ensure "name" is required (not null)
            if (name == null)
            {
                throw new ArgumentNullException("name is a required property for EventTypeOut and cannot be null");
            }
            this.Name = name;
            this.UpdatedAt = updatedAt;
            this.Archived = archived;
            this.FeatureFlag = featureFlag;
            this.GroupName = groupName;
            this.Schemas = schemas;
        }

        /// <summary>
        /// Gets or Sets Archived
        /// </summary>
        /*
        <example>false</example>
        */
        [DataMember(Name = "archived", EmitDefaultValue = true)]
        public bool Archived { get; set; }

        /// <summary>
        /// Gets or Sets CreatedAt
        /// </summary>
        [DataMember(Name = "createdAt", IsRequired = true, EmitDefaultValue = true)]
        public DateTime CreatedAt { get; set; }

        /// <summary>
        /// Gets or Sets Deprecated
        /// </summary>
        [DataMember(Name = "deprecated", IsRequired = true, EmitDefaultValue = true)]
        public bool Deprecated { get; set; }

        /// <summary>
        /// Gets or Sets Description
        /// </summary>
        /*
        <example>A user has signed up</example>
        */
        [DataMember(Name = "description", IsRequired = true, EmitDefaultValue = true)]
        public string Description { get; set; }

        /// <summary>
        /// Gets or Sets FeatureFlag
        /// </summary>
        /*
        <example>cool-new-feature</example>
        */
        [DataMember(Name = "featureFlag", EmitDefaultValue = true)]
        public string FeatureFlag { get; set; }

        /// <summary>
        /// The event type group&#39;s name
        /// </summary>
        /// <value>The event type group&#39;s name</value>
        /*
        <example>user</example>
        */
        [DataMember(Name = "groupName", EmitDefaultValue = true)]
        public string GroupName { get; set; }

        /// <summary>
        /// The event type&#39;s name
        /// </summary>
        /// <value>The event type&#39;s name</value>
        /*
        <example>user.signup</example>
        */
        [DataMember(Name = "name", IsRequired = true, EmitDefaultValue = true)]
        public string Name { get; set; }

        /// <summary>
        /// The schema for the event type for a specific version as a JSON schema.
        /// </summary>
        /// <value>The schema for the event type for a specific version as a JSON schema.</value>
        /*
        <example>{&quot;1&quot;:{&quot;description&quot;:&quot;An invoice was paid by a user&quot;,&quot;properties&quot;:{&quot;invoiceId&quot;:{&quot;description&quot;:&quot;The invoice id&quot;,&quot;type&quot;:&quot;string&quot;},&quot;userId&quot;:{&quot;description&quot;:&quot;The user id&quot;,&quot;type&quot;:&quot;string&quot;}},&quot;required&quot;:[&quot;invoiceId&quot;,&quot;userId&quot;],&quot;title&quot;:&quot;Invoice Paid Event&quot;,&quot;type&quot;:&quot;object&quot;}}</example>
        */
        [DataMember(Name = "schemas", EmitDefaultValue = true)]
        public Object Schemas { get; set; }

        /// <summary>
        /// Gets or Sets UpdatedAt
        /// </summary>
        [DataMember(Name = "updatedAt", IsRequired = true, EmitDefaultValue = true)]
        public DateTime UpdatedAt { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EventTypeOut {\n");
            sb.Append("  Archived: ").Append(Archived).Append("\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append("\n");
            sb.Append("  Deprecated: ").Append(Deprecated).Append("\n");
            sb.Append("  Description: ").Append(Description).Append("\n");
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append("\n");
            sb.Append("  GroupName: ").Append(GroupName).Append("\n");
            sb.Append("  Name: ").Append(Name).Append("\n");
            sb.Append("  Schemas: ").Append(Schemas).Append("\n");
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append("\n");
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
            // FeatureFlag (string) maxLength
            if (this.FeatureFlag != null && this.FeatureFlag.Length > 256)
            {
                yield return new ValidationResult("Invalid value for FeatureFlag, length must be less than 256.", new [] { "FeatureFlag" });
            }

            if (this.FeatureFlag != null) {
                // FeatureFlag (string) pattern
                Regex regexFeatureFlag = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexFeatureFlag.Match(this.FeatureFlag).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for FeatureFlag, must match a pattern of " + regexFeatureFlag, new [] { "FeatureFlag" });
                }
            }

            // GroupName (string) maxLength
            if (this.GroupName != null && this.GroupName.Length > 256)
            {
                yield return new ValidationResult("Invalid value for GroupName, length must be less than 256.", new [] { "GroupName" });
            }

            if (this.GroupName != null) {
                // GroupName (string) pattern
                Regex regexGroupName = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexGroupName.Match(this.GroupName).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for GroupName, must match a pattern of " + regexGroupName, new [] { "GroupName" });
                }
            }

            // Name (string) maxLength
            if (this.Name != null && this.Name.Length > 256)
            {
                yield return new ValidationResult("Invalid value for Name, length must be less than 256.", new [] { "Name" });
            }

            if (this.Name != null) {
                // Name (string) pattern
                Regex regexName = new Regex(@"^[a-zA-Z0-9\-_.]+$", RegexOptions.CultureInvariant);
                if (!regexName.Match(this.Name).Success)
                {
                    yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for Name, must match a pattern of " + regexName, new [] { "Name" });
                }
            }

            yield break;
        }
    }

}
