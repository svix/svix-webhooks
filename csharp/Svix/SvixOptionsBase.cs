using Newtonsoft.Json;

namespace Svix
{
    public abstract class SvixOptionsBase
    {
#pragma warning disable CA1822

        public Dictionary<string, string> SerializeParams(Dictionary<string, object?> rawParams)
#pragma warning restore CA1822

        {
            var dict = new Dictionary<string, string>();
            foreach (KeyValuePair<string, object?> entry in rawParams)
            {
                if (entry.Value == null) { }
                else if (IsGenericList(entry.Value))
                {
                    var list = (System.Collections.IEnumerable)entry.Value;
                    dict[entry.Key] = String.Join(",", list.Cast<object>());
                }
                else
                {
                    var jsonStr = JsonConvert.SerializeObject(entry.Value);
                    // enum values get serialized as single value json objects (just a string)
                    // so I need to strip the start and ending quotes.
                    // FIXME: find a better way to do this :(
                    if (jsonStr.EndsWith('"') && jsonStr.StartsWith('"'))
                    {
                        jsonStr = jsonStr.TrimEnd('"');
                        jsonStr = jsonStr.TrimStart('"');
                    }
                    dict[entry.Key] = jsonStr;
                }
            }

            return dict;
        }

#pragma warning disable CA1822
        public Dictionary<string, string> QueryParams()
        {
            return new Dictionary<string, string>();
        }

        public Dictionary<string, string> HeaderParams()
        {
            return new Dictionary<string, string>();
        }
#pragma warning restore CA1822
        private static bool IsGenericList(object o)
        {
            var oType = o.GetType();
            return (oType.IsGenericType && (oType.GetGenericTypeDefinition() == typeof(List<>)));
        }
    }
}
