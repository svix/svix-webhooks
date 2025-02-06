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
using System.Reflection;

namespace Svix.Model
{
    /// <summary>
    /// A copy of [&#x60;backgroundtask::Data&#x60;], but serialized with camelCase fields for customers.
    /// </summary>
    [JsonConverter(typeof(DataJsonConverter))]
    [DataContract(Name = "Data")]
    public partial class Data : AbstractOpenAPISchema, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="Data" /> class
        /// with the <see cref="DataAnyOf" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of DataAnyOf.</param>
        public Data(DataAnyOf actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Data" /> class
        /// with the <see cref="DataAnyOf1" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of DataAnyOf1.</param>
        public Data(DataAnyOf1 actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Data" /> class
        /// with the <see cref="DataAnyOf2" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of DataAnyOf2.</param>
        public Data(DataAnyOf2 actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Data" /> class
        /// with the <see cref="DataAnyOf3" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of DataAnyOf3.</param>
        public Data(DataAnyOf3 actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }


        private Object _actualInstance;

        /// <summary>
        /// Gets or Sets ActualInstance
        /// </summary>
        public override Object ActualInstance
        {
            get
            {
                return _actualInstance;
            }
            set
            {
                if (value.GetType() == typeof(DataAnyOf))
                {
                    _actualInstance = value;
                }
                else if (value.GetType() == typeof(DataAnyOf1))
                {
                    _actualInstance = value;
                }
                else if (value.GetType() == typeof(DataAnyOf2))
                {
                    _actualInstance = value;
                }
                else if (value.GetType() == typeof(DataAnyOf3))
                {
                    _actualInstance = value;
                }
                else
                {
                    throw new ArgumentException("Invalid instance found. Must be the following types: DataAnyOf, DataAnyOf1, DataAnyOf2, DataAnyOf3");
                }
            }
        }

        /// <summary>
        /// Get the actual instance of `DataAnyOf`. If the actual instance is not `DataAnyOf`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of DataAnyOf</returns>
        public DataAnyOf GetDataAnyOf()
        {
            return (DataAnyOf)ActualInstance;
        }

        /// <summary>
        /// Get the actual instance of `DataAnyOf1`. If the actual instance is not `DataAnyOf1`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of DataAnyOf1</returns>
        public DataAnyOf1 GetDataAnyOf1()
        {
            return (DataAnyOf1)ActualInstance;
        }

        /// <summary>
        /// Get the actual instance of `DataAnyOf2`. If the actual instance is not `DataAnyOf2`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of DataAnyOf2</returns>
        public DataAnyOf2 GetDataAnyOf2()
        {
            return (DataAnyOf2)ActualInstance;
        }

        /// <summary>
        /// Get the actual instance of `DataAnyOf3`. If the actual instance is not `DataAnyOf3`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of DataAnyOf3</returns>
        public DataAnyOf3 GetDataAnyOf3()
        {
            return (DataAnyOf3)ActualInstance;
        }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class Data {\n");
            sb.Append("  ActualInstance: ").Append(ActualInstance).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public override string ToJson()
        {
            return JsonConvert.SerializeObject(ActualInstance, Data.SerializerSettings);
        }

        /// <summary>
        /// Converts the JSON string into an instance of Data
        /// </summary>
        /// <param name="jsonString">JSON string</param>
        /// <returns>An instance of Data</returns>
        public static Data FromJson(string jsonString)
        {
            Data newData = null;

            if (string.IsNullOrEmpty(jsonString))
            {
                return newData;
            }

            try
            {
                newData = new Data(JsonConvert.DeserializeObject<DataAnyOf>(jsonString, Data.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newData;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into DataAnyOf: {1}", jsonString, exception.ToString()));
            }

            try
            {
                newData = new Data(JsonConvert.DeserializeObject<DataAnyOf1>(jsonString, Data.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newData;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into DataAnyOf1: {1}", jsonString, exception.ToString()));
            }

            try
            {
                newData = new Data(JsonConvert.DeserializeObject<DataAnyOf2>(jsonString, Data.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newData;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into DataAnyOf2: {1}", jsonString, exception.ToString()));
            }

            try
            {
                newData = new Data(JsonConvert.DeserializeObject<DataAnyOf3>(jsonString, Data.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newData;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into DataAnyOf3: {1}", jsonString, exception.ToString()));
            }

            // no match found, throw an exception
            throw new InvalidDataException("The JSON string `" + jsonString + "` cannot be deserialized into any schema defined.");
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

    /// <summary>
    /// Custom JSON converter for Data
    /// </summary>
    public class DataJsonConverter : JsonConverter
    {
        /// <summary>
        /// To write the JSON string
        /// </summary>
        /// <param name="writer">JSON writer</param>
        /// <param name="value">Object to be converted into a JSON string</param>
        /// <param name="serializer">JSON Serializer</param>
        public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
        {
            writer.WriteRawValue((string)(typeof(Data).GetMethod("ToJson").Invoke(value, null)));
        }

        /// <summary>
        /// To convert a JSON string into an object
        /// </summary>
        /// <param name="reader">JSON reader</param>
        /// <param name="objectType">Object type</param>
        /// <param name="existingValue">Existing value</param>
        /// <param name="serializer">JSON Serializer</param>
        /// <returns>The object converted from the JSON string</returns>
        public override object ReadJson(JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
        {
            switch(reader.TokenType) 
            {
                case JsonToken.StartObject:
                    return Data.FromJson(JObject.Load(reader).ToString(Formatting.None));
                case JsonToken.StartArray:
                    return Data.FromJson(JArray.Load(reader).ToString(Formatting.None));
                default:
                    return null;
            }
        }

        /// <summary>
        /// Check if the object can be converted
        /// </summary>
        /// <param name="objectType">Object type</param>
        /// <returns>True if the object can be converted</returns>
        public override bool CanConvert(Type objectType)
        {
            return false;
        }
    }

}
