using System;
using System.Runtime.CompilerServices;

namespace Svix
{
    internal static class Utils
    {
        // Borrowed from Stripe-dotnet https://github.com/stripe/stripe-dotnet/blob/7b62c461d7c0cf2c9e06dce5e564b374a9d232e0/src/Stripe.net/Infrastructure/StringUtils.cs#L30
        // basically identical to SecureCompare from Rails::ActiveSupport used in our ruby lib
        [MethodImpl(MethodImplOptions.NoOptimization)]
        public static bool SecureCompare(ReadOnlySpan<char> a, ReadOnlySpan<char> b)
        {
            if (a.Length != b.Length)
            {
                return false;
            }

            var result = 0;
            for (var i = 0; i < a.Length; i++)
            {
                result |= a[i] ^ b[i];
            }

            return result == 0;
        }

        internal static string DefaultServerUrlFromToken(string token)
        {
            string[] tokenParts = token.Split('.');
            string region = tokenParts[tokenParts.Length - 1];

            if (region == "us")
            {
                return "https://api.us.svix.com";
            }
            else if (region == "eu")
            {
                return "https://api.eu.svix.com";
            }
            else if (region == "in")
            {
                return "https://api.in.svix.com";
            }
            else
            {
                return "https://api.svix.com";
            }
        }
    }
}
