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
    /// EnvironmentOut
    /// </summary>
    [DataContract(Name = "EnvironmentOut")]
    public partial class EnvironmentOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EnvironmentOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EnvironmentOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EnvironmentOut" /> class.
        /// </summary>
        /// <param name="createdAt">createdAt (required).</param>
        /// <param name="eventTypes">eventTypes (required).</param>
        /// <param name="settings">settings (required).</param>
        /// <param name="transformationTemplates">transformationTemplates (required).</param>
        /// <param name="varVersion">varVersion (default to 1).</param>
        public EnvironmentOut(DateTime createdAt = default(DateTime), List<EventTypeOut> eventTypes = default(List<EventTypeOut>), Object settings = default(Object), List<TemplateOut> transformationTemplates = default(List<TemplateOut>), int varVersion = 1)
        {
            this.CreatedAt = createdAt;
            // to ensure "eventTypes" is required (not null)
            if (eventTypes == null)
            {
                throw new ArgumentNullException("eventTypes is a required property for EnvironmentOut and cannot be null");
            }
            this.EventTypes = eventTypes;
            // to ensure "settings" is required (not null)
            if (settings == null)
            {
                throw new ArgumentNullException("settings is a required property for EnvironmentOut and cannot be null");
            }
            this.Settings = settings;
            // to ensure "transformationTemplates" is required (not null)
            if (transformationTemplates == null)
            {
                throw new ArgumentNullException("transformationTemplates is a required property for EnvironmentOut and cannot be null");
            }
            this.TransformationTemplates = transformationTemplates;
            this.VarVersion = varVersion;
        }

        /// <summary>
        /// Gets or Sets CreatedAt
        /// </summary>
        [DataMember(Name = "createdAt", IsRequired = true, EmitDefaultValue = true)]
        public DateTime CreatedAt { get; set; }

        /// <summary>
        /// Gets or Sets EventTypes
        /// </summary>
        [DataMember(Name = "eventTypes", IsRequired = true, EmitDefaultValue = true)]
        public List<EventTypeOut> EventTypes { get; set; }

        /// <summary>
        /// Gets or Sets Settings
        /// </summary>
        [DataMember(Name = "settings", IsRequired = true, EmitDefaultValue = true)]
        public Object Settings { get; set; }

        /// <summary>
        /// Gets or Sets TransformationTemplates
        /// </summary>
        [DataMember(Name = "transformationTemplates", IsRequired = true, EmitDefaultValue = true)]
        public List<TemplateOut> TransformationTemplates { get; set; }

        /// <summary>
        /// Gets or Sets VarVersion
        /// </summary>
        [DataMember(Name = "version", EmitDefaultValue = false)]
        public int VarVersion { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EnvironmentOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append("\n");
            sb.Append("  EventTypes: ").Append(EventTypes).Append("\n");
            sb.Append("  Settings: ").Append(Settings).Append("\n");
            sb.Append("  TransformationTemplates: ").Append(TransformationTemplates).Append("\n");
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
            yield break;
        }
    }

}
