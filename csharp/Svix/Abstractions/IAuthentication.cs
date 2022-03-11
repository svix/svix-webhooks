namespace Svix.Abstractions
{
    public interface IAuthentication
    {
        void GetDashboardAccess();
        
        void GetDashboardAccessAsync();

        void Logout();
        
        void LogoutAsync();
    }
}