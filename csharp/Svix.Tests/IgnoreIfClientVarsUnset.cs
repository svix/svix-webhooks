using System;
using Xunit;

namespace Svix.Tests;

public sealed class IgnoreIfClientVarsUnset : FactAttribute
{
    public IgnoreIfClientVarsUnset()
    {
        if (
            System.Environment.GetEnvironmentVariable("SVIX_TOKEN") == null
            && System.Environment.GetEnvironmentVariable("SVIX_SERVER_URL") == null
        )
        {
            Skip =
                "Test client cannot be constructed when environment variable SVIX_TOKEN or SVIX_SERVER_URL is unset.";
        }
    }
}
