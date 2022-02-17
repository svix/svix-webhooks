namespace Svix.Models
{
    public sealed class SvixClientOptions
    {
        public string AccessToken { get; set; }
        
        public string ServerUrl { get; set; }

        public bool Throw { get; set; }
    }
}