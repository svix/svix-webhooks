using System;
using Svix.Abstractions;
using Svix.Api;

namespace Svix
{
    public sealed class Authentication : SvixResourceBase, IAuthentication
    {
        private readonly IAuthenticationApi _authenticationApi;
        
        public Authentication(ISvixClient svixClient, IAuthenticationApi authenticationApi) 
            : base(svixClient)
        {
            _authenticationApi = authenticationApi ?? throw new ArgumentNullException(nameof(authenticationApi));
        }

        public void GetDashboardAccess()
        {
            throw new System.NotImplementedException();
        }

        public void GetDashboardAccessAsync()
        {
            throw new NotImplementedException();
        }

        public void Logout()
        {
            throw new System.NotImplementedException();
        }

        public void LogoutAsync()
        {
            throw new NotImplementedException();
        }
    }
}