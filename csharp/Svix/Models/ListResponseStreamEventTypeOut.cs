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
    /// ListResponseStreamEventTypeOut
    /// </summary>
    [DataContract(Name = "ListResponse_StreamEventTypeOut_")]
    public partial class ListResponseStreamEventTypeOut : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ListResponseStreamEventTypeOut" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected ListResponseStreamEventTypeOut() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="ListResponseStreamEventTypeOut" /> class.
        /// </summary>
        /// <param name="data">data (required).</param>
        /// <param name="done">done (required).</param>
        /// <param name="iterator">iterator (required).</param>
        /// <param name="prevIterator">prevIterator.</param>
        public ListResponseStreamEventTypeOut(List<StreamEventTypeOut> data = default(List<StreamEventTypeOut>), bool done = default(bool), string iterator = default(string), string prevIterator = default(string))
        {
            // to ensure "data" is required (not null)
            if (data == null)
            {
                throw new ArgumentNullException("data is a required property for ListResponseStreamEventTypeOut and cannot be null");
            }
            this.Data = data;
            this.Done = done;
            // to ensure "iterator" is required (not null)
            if (iterator == null)
            {
                throw new ArgumentNullException("iterator is a required property for ListResponseStreamEventTypeOut and cannot be null");
            }
            this.Iterator = iterator;
            this.PrevIterator = prevIterator;
        }

        /// <summary>
        /// Gets or Sets Data
        /// </summary>
        [DataMember(Name = "data", IsRequired = true, EmitDefaultValue = true)]
        public List<StreamEventTypeOut> Data { get; set; }

        /// <summary>
        /// Gets or Sets Done
        /// </summary>
        [DataMember(Name = "done", IsRequired = true, EmitDefaultValue = true)]
        public bool Done { get; set; }

        /// <summary>
        /// Gets or Sets Iterator
        /// </summary>
        /*
        <example>iterator</example>
        */
        [DataMember(Name = "iterator", IsRequired = true, EmitDefaultValue = true)]
        public string Iterator { get; set; }

        /// <summary>
        /// Gets or Sets PrevIterator
        /// </summary>
        /*
        <example>-iterator</example>
        */
        [DataMember(Name = "prevIterator", EmitDefaultValue = true)]
        public string PrevIterator { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class ListResponseStreamEventTypeOut {\n");
            sb.Append("  Data: ").Append(Data).Append("\n");
            sb.Append("  Done: ").Append(Done).Append("\n");
            sb.Append("  Iterator: ").Append(Iterator).Append("\n");
            sb.Append("  PrevIterator: ").Append(PrevIterator).Append("\n");
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
