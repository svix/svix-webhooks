using System;
using Svix.Abstractions;
using Svix.Api;

namespace Svix
{
    public sealed class Endpoint : SvixResourceBase, IEndpoint
    {
        private readonly IEndpointApi _endpointApi;
        
        public Endpoint(ISvixClient svixClient, IEndpointApi endpoingApi) 
            : base(svixClient)
        {
            _endpointApi = endpoingApi ?? throw new ArgumentNullException(nameof(_endpointApi));
        }

        public void Create()
        {
            throw new NotImplementedException();
        }

        public void CreateAsync()
        {
            throw new NotImplementedException();
        }

        public void Delete()
        {
            throw new NotImplementedException();
        }

        public void DeleteAsync()
        {
            throw new NotImplementedException();
        }

        public void Get()
        {
            throw new NotImplementedException();
        }

        public void GetAsync()
        {
            throw new NotImplementedException();
        }

        public void GetHeaders()
        {
            throw new NotImplementedException();
        }

        public void GetHeadersAsync()
        {
            throw new NotImplementedException();
        }

        public void GetSecret()
        {
            throw new NotImplementedException();
        }

        public void GetSecretAsync()
        {
            throw new NotImplementedException();
        }

        public void List()
        {
            throw new NotImplementedException();
        }

        public void ListAsync()
        {
            throw new NotImplementedException();
        }

        public void PatchHeaders()
        {
            throw new NotImplementedException();
        }

        public void PatchHeadersAsync()
        {
            throw new NotImplementedException();
        }

        public void Recover()
        {
            throw new NotImplementedException();
        }

        public void RecoverAsync()
        {
            throw new NotImplementedException();
        }

        public void RotateSecret()
        {
            throw new NotImplementedException();
        }

        public void RotateSecretAsync()
        {
            throw new NotImplementedException();
        }

        public void Update()
        {
            throw new NotImplementedException();
        }

        public void UpdateAsync()
        {
            throw new NotImplementedException();
        }

        public void UpdateHeaders()
        {
            throw new NotImplementedException();
        }

        public void UpdateHeadersAsync()
        {
            throw new NotImplementedException();
        }
    }
}