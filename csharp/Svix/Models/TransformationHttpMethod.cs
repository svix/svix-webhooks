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
    /// Defines TransformationHttpMethod
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum TransformationHttpMethod
    {
        /// <summary>
        /// Enum POST for value: POST
        /// </summary>
        [EnumMember(Value = "POST")]
        POST = 1,

        /// <summary>
        /// Enum PUT for value: PUT
        /// </summary>
        [EnumMember(Value = "PUT")]
        PUT = 2,

        /// <summary>
        /// Enum PATCH for value: PATCH
        /// </summary>
        [EnumMember(Value = "PATCH")]
        PATCH = 3
    }

}
