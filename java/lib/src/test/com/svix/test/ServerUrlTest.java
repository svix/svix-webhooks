package com.svix.test;

import static org.junit.Assert.assertEquals;

import com.svix.Svix;
import com.svix.SvixOptions;

import org.junit.Test;

public class ServerUrlTest {
    @Test
    public void testServerUrlTrailingSlashesAreStripped() {
        SvixOptions opts = new SvixOptions();
        opts.setServerUrl("https://api.example.com/");
        assertEquals("https://api.example.com", opts.getServerUrl());

        opts.setServerUrl("https://api.example.com///");
        assertEquals("https://api.example.com", opts.getServerUrl());

        opts.setServerUrl("https://api.example.com/prefix/");
        assertEquals("https://api.example.com/prefix", opts.getServerUrl());
    }

    @Test
    public void testServerUrlWithoutTrailingSlashIsUnchanged() {
        SvixOptions opts = new SvixOptions();
        opts.setServerUrl("https://api.example.com");
        assertEquals("https://api.example.com", opts.getServerUrl());
    }

    @Test
    public void testDefaultAndRegionalServerUrls() {
        Svix defaultSvx = new Svix("token");
        assertEquals("https://api.svix.com", defaultSvx.getHttpClient().getBaseUrl().toString().replaceAll("/+$", ""));

        Svix regionalSvx = new Svix("testsk.eu");
        assertEquals("https://api.eu.svix.com", regionalSvx.getHttpClient().getBaseUrl().toString().replaceAll("/+$", ""));
    }
}
