namespace Svix.Abstractions
{
    public interface IMessage
    {
        void Create();
        
        void CreateAsync();

        void Get();
        
        void GetAsync();

        void List();
        
        void ListAsync();
    }
}