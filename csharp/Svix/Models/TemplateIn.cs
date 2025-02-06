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
    /// TemplateIn
    /// </summary>
    [DataContract(Name = "TemplateIn")]
    public partial class TemplateIn : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets Kind
        /// </summary>
        [DataMember(Name = "kind", EmitDefaultValue = false)]
        public TransformationTemplateKind? Kind { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="TemplateIn" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected TemplateIn() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="TemplateIn" /> class.
        /// </summary>
        /// <param name="description">description (default to &quot;&quot;).</param>
        /// <param name="featureFlag">featureFlag.</param>
        /// <param name="filterTypes">filterTypes.</param>
        /// <param name="instructions">instructions (default to &quot;&quot;).</param>
        /// <param name="instructionsLink">instructionsLink.</param>
        /// <param name="kind">kind.</param>
        /// <param name="logo">logo (required).</param>
        /// <param name="name">name (required).</param>
        /// <param name="transformation">transformation (required).</param>
        public TemplateIn(string description = @"", string featureFlag = default(string), List<string> filterTypes = default(List<string>), string instructions = @"", string instructionsLink = default(string), TransformationTemplateKind? kind = default(TransformationTemplateKind?), string logo = default(string), string name = default(string), string transformation = default(string))
        {
            // to ensure "logo" is required (not null)
            if (logo == null)
            {
                throw new ArgumentNullException("logo is a required property for TemplateIn and cannot be null");
            }
            this.Logo = logo;
            // to ensure "name" is required (not null)
            if (name == null)
            {
                throw new ArgumentNullException("name is a required property for TemplateIn and cannot be null");
            }
            this.Name = name;
            // to ensure "transformation" is required (not null)
            if (transformation == null)
            {
                throw new ArgumentNullException("transformation is a required property for TemplateIn and cannot be null");
            }
            this.Transformation = transformation;
            // use default value if no "description" provided
            this.Description = description ?? @"";
            this.FeatureFlag = featureFlag;
            this.FilterTypes = filterTypes;
            // use default value if no "instructions" provided
            this.Instructions = instructions ?? @"";
            this.InstructionsLink = instructionsLink;
            this.Kind = kind;
        }

        /// <summary>
        /// Gets or Sets Description
        /// </summary>
        [DataMember(Name = "description", EmitDefaultValue = false)]
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
        /// Gets or Sets FilterTypes
        /// </summary>
        /*
        <example>[&quot;user.signup&quot;,&quot;user.deleted&quot;]</example>
        */
        [DataMember(Name = "filterTypes", EmitDefaultValue = true)]
        public List<string> FilterTypes { get; set; }

        /// <summary>
        /// Gets or Sets Instructions
        /// </summary>
        [DataMember(Name = "instructions", EmitDefaultValue = false)]
        public string Instructions { get; set; }

        /// <summary>
        /// Gets or Sets InstructionsLink
        /// </summary>
        [DataMember(Name = "instructionsLink", EmitDefaultValue = true)]
        public string InstructionsLink { get; set; }

        /// <summary>
        /// Gets or Sets Logo
        /// </summary>
        [DataMember(Name = "logo", IsRequired = true, EmitDefaultValue = true)]
        public string Logo { get; set; }

        /// <summary>
        /// Gets or Sets Name
        /// </summary>
        [DataMember(Name = "name", IsRequired = true, EmitDefaultValue = true)]
        public string Name { get; set; }

        /// <summary>
        /// Gets or Sets Transformation
        /// </summary>
        [DataMember(Name = "transformation", IsRequired = true, EmitDefaultValue = true)]
        public string Transformation { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class TemplateIn {\n");
            sb.Append("  Description: ").Append(Description).Append("\n");
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append("\n");
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append("\n");
            sb.Append("  Instructions: ").Append(Instructions).Append("\n");
            sb.Append("  InstructionsLink: ").Append(InstructionsLink).Append("\n");
            sb.Append("  Kind: ").Append(Kind).Append("\n");
            sb.Append("  Logo: ").Append(Logo).Append("\n");
            sb.Append("  Name: ").Append(Name).Append("\n");
            sb.Append("  Transformation: ").Append(Transformation).Append("\n");
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

            // Transformation (string) maxLength
            if (this.Transformation != null && this.Transformation.Length > 51200)
            {
                yield return new ValidationResult("Invalid value for Transformation, length must be less than 51200.", new [] { "Transformation" });
            }

            // Transformation (string) minLength
            if (this.Transformation != null && this.Transformation.Length < 10)
            {
                yield return new ValidationResult("Invalid value for Transformation, length must be greater than 10.", new [] { "Transformation" });
            }

            yield break;
        }
    }

}
