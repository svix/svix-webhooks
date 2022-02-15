namespace Svix.Models
{
    public sealed class SvixClientOptions
    {
        public string AccessToken { get; set; }
        
        public string ServiceUrl { get; set; }

        public bool Throw { get; set; }
    }
}