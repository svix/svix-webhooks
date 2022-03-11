namespace Svix.Abstractions
{
    public interface IEventType
    {
        void Archive();
        
        void ArchiveAsync();

        void Create();
        
        void CreateAsync();

        void Get();
        
        void GetAsync();

        void List();
        
        void ListAsync();

        void Update();
        
        void UpdateAsync();
    }
}