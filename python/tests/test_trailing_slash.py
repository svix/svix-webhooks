import pytest
from svix.api import Svix, SvixOptions


def test_trailing_slash_handling():
    """Test that trailing slashes are properly removed from server URLs."""
    
    # Test with trailing slash
    svix_with_slash = Svix("key", SvixOptions(server_url="http://localhost:8000/"))
    assert svix_with_slash._client.base_url == "http://localhost:8000"
    
    # Test without trailing slash
    svix_without_slash = Svix("key", SvixOptions(server_url="http://localhost:8000"))
    assert svix_without_slash._client.base_url == "http://localhost:8000"
    
    # Test with multiple trailing slashes
    svix_multiple_slashes = Svix("key", SvixOptions(server_url="http://localhost:8000///"))
    assert svix_multiple_slashes._client.base_url == "http://localhost:8000"


def test_regional_url_trailing_slash():
    """Test that regional URLs are handled correctly."""
    
    # Test EU region token (regional URL should not have trailing slash)
    svix_eu = Svix("test.eu.token")
    assert svix_eu._client.base_url == "https://api.eu.svix.com"
    
    # Test US region token
    svix_us = Svix("test.us.token") 
    assert svix_us._client.base_url == "https://api.us.svix.com"


def test_default_url_no_trailing_slash():
    """Test that default URL doesn't have trailing slash."""
    
    svix_default = Svix("key")
    assert svix_default._client.base_url == "https://api.svix.com"