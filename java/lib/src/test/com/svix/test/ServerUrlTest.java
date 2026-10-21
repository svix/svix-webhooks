package com.svix.test;

import static org.junit.Assert.assertEquals;

import com.svix.Svix;
import com.svix.SvixOptions;

import org.junit.Test;

public class ServerUrlTest {
    @Test
    public void testServerUrlTrailingSlashesAreStripped() {
        SvixOptions opts1 = new SvixOptions();
        opts1.setServerUrl("https://api.example.com/");
        Svix svx1 = new Svix("token", opts1);
        assertEquals("https://api.example.com/", svx1.getHttpClient().getBaseUrl().toString());

        SvixOptions opts2 = new SvixOptions();
        opts2.setServerUrl("https://api.example.com///");
        Svix svx2 = new Svix("token", opts2);
        assertEquals("https://api.example.com/", svx2.getHttpClient().getBaseUrl().toString());

        SvixOptions opts3 = new SvixOptions();
        opts3.setServerUrl("https://api.example.com/prefix/");
        Svix svx3 = new Svix("token", opts3);
        assertEquals("https://api.example.com/prefix", svx3.getHttpClient().getBaseUrl().toString());

        SvixOptions opts4 = new SvixOptions();
        opts4.setServerUrl("https://api.example.com/prefix///");
        Svix svx4 = new Svix("token", opts4);
        assertEquals("https://api.example.com/prefix", svx4.getHttpClient().getBaseUrl().toString());
    }

    @Test
    public void testServerUrlWithoutTrailingSlashIsUnchanged() {
        SvixOptions opts1 = new SvixOptions();
        opts1.setServerUrl("https://api.example.com");
        Svix svx1 = new Svix("token", opts1);
        assertEquals("https://api.example.com/", svx1.getHttpClient().getBaseUrl().toString());

        SvixOptions opts2 = new SvixOptions();
        opts2.setServerUrl("https://api.example.com/prefix");
        Svix svx2 = new Svix("token", opts2);
        assertEquals("https://api.example.com/prefix", svx2.getHttpClient().getBaseUrl().toString());
    }

    @Test
    public void testDefaultAndRegionalServerUrls() {
        Svix defaultSvx = new Svix("token");
        assertEquals("https://api.svix.com/", defaultSvx.getHttpClient().getBaseUrl().toString());

        Svix regionalSvx = new Svix("testsk.eu");
        assertEquals("https://api.eu.svix.com/", regionalSvx.getHttpClient().getBaseUrl().toString());
    }

    @Test
    public void testConstructedRequestUrls() {
        Svix defaultSvx = new Svix("token");
        assertEquals(
                "https://api.svix.com/api/v1/app",
                defaultSvx.getHttpClient().newUrlBuilder().encodedPath("/api/v1/app").build().toString());

        Svix regionalSvx = new Svix("testsk.eu");
        assertEquals(
                "https://api.eu.svix.com/api/v1/app",
                regionalSvx.getHttpClient().newUrlBuilder().encodedPath("/api/v1/app").build().toString());

        SvixOptions optsWithSlash = new SvixOptions();
        optsWithSlash.setServerUrl("https://api.example.com/");
        Svix svxWithSlash = new Svix("token", optsWithSlash);
        assertEquals(
                "https://api.example.com/api/v1/app",
                svxWithSlash.getHttpClient().newUrlBuilder().encodedPath("/api/v1/app").build().toString());

        SvixOptions optsWithMultiSlash = new SvixOptions();
        optsWithMultiSlash.setServerUrl("https://api.example.com///");
        Svix svxWithMultiSlash = new Svix("token", optsWithMultiSlash);
        assertEquals(
                "https://api.example.com/api/v1/app",
                svxWithMultiSlash.getHttpClient().newUrlBuilder().encodedPath("/api/v1/app").build().toString());
    }
}
