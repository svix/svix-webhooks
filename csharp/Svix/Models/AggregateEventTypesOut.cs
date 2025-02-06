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
    /// AggregateEventTypesOut
    /// </summary>
    [DataContract(Name = "AggregateEventTypesOut")]
    public partial class AggregateEventTypesOut : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets Status
        /// </summary>
        [DataMember(Name = "status", IsRequired = true, EmitDefaultValue = true)]
        public BackgroundTaskStatus Status { get; set; }

        /// <summary>
        /// Gets or Sets Task
        /// </summary>
        [DataMember(Name = "task", IsRequired = true, EmitDefaultValue = true)]
        public BackgroundTaskType Task { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="AggregateEventTypesOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected AggregateEventTypesOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="AggregateEventTypesOut" /> class.
        /// </summary>
        /// <param name="id">id (required).</param>
        /// <param name="status">status (required).</param>
        /// <param name="task">task (required).</param>
        public AggregateEventTypesOut(string id = default(string), BackgroundTaskStatus status = default(BackgroundTaskStatus), BackgroundTaskType task = default(BackgroundTaskType))
        {
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for AggregateEventTypesOut and cannot be null");
            }
            this.Id = id;
            this.Status = status;
            this.Task = task;
        }

        /// <summary>
        /// Gets or Sets Id
        /// </summary>
        [DataMember(Name = "id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class AggregateEventTypesOut {\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  Status: ").Append(Status).Append("\n");
            sb.Append("  Task: ").Append(Task).Append("\n");
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
