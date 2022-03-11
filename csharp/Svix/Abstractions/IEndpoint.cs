namespace Svix.Abstractions
{
    public interface IEndpoint
    {
        void Create();

        void CreateAsync();

        void Delete();

        void DeleteAsync();
        
        void Get();

        void GetAsync();

        void GetHeaders();

        void GetHeadersAsync();
        
        void GetSecret();

        void GetSecretAsync();
        
        void List();

        void ListAsync();

        void PatchHeaders();

        void PatchHeadersAsync();
        
        void Recover();

        void RecoverAsync();
        
        void RotateSecret();

        void RotateSecretAsync();

        void Update();

        void UpdateAsync();

        void UpdateHeaders();

        void UpdateHeadersAsync();
    }
}