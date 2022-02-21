namespace Svix.Abstractions
{
    public interface ISvixOptions
    {
        public string ServerUrl { get; }

        public bool Throw { get; }
    }
}