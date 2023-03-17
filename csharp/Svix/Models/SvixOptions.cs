using Svix.Abstractions;

namespace Svix.Models
{
    public sealed class SvixOptions : ISvixOptions
    {
        public string ServerUrl { get; }

        public bool Throw { get; }

        private SvixOptions()
        {
            // empty
        }

        public SvixOptions(string serverUrl, bool bThrow = true)
        {
            ServerUrl = serverUrl;
            Throw = bThrow;
        }
    }
}