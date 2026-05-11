package com.svix.test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertThrows;

import com.svix.AutoConfig;

import org.junit.Test;

import java.nio.charset.StandardCharsets;
import java.util.Base64;

public class AutoConfigTest {
    @Test
    public void decodeTokenParsesV1Payload() throws Exception {
        String json =
                "{\"aid\":\"app_1\",\"eid\":\"ep_2\",\"surl\":\"https://api.example.test\",\"esec\":\"whsec_Zm9v\",\"tok\":\"sk_test_xyz\"}";
        String token =
                AutoConfig.AUTOCONFIG_TOKEN_PREFIX_V1
                        + Base64.getEncoder().encodeToString(json.getBytes(StandardCharsets.UTF_8));

        AutoConfig.DecodedTokenContent content = AutoConfig.decodeToken(token);

        assertEquals("app_1", content.getAppId());
        assertEquals("ep_2", content.getEndpointId());
        assertEquals("https://api.example.test", content.getServerUrl());
        assertEquals("whsec_Zm9v", content.getEndpointSecret());
        assertEquals("sk_test_xyz", content.getTokenPlaintext());
    }

    @Test
    public void decodeTokenRejectsBadPrefix() {
        String json =
                "{\"aid\":\"a\",\"eid\":\"e\",\"surl\":\"https://x\",\"esec\":\"whsec_Zm9v\",\"tok\":\"t\"}";
        String token =
                "wrong_" + Base64.getEncoder().encodeToString(json.getBytes(StandardCharsets.UTF_8));

        assertThrows(AutoConfig.InvalidTokenException.class, () -> AutoConfig.decodeToken(token));
    }

    @Test
    public void decodeTokenRejectsInvalidJson() {
        String token =
                AutoConfig.AUTOCONFIG_TOKEN_PREFIX_V1
                        + Base64.getEncoder()
                                .encodeToString("not json".getBytes(StandardCharsets.UTF_8));

        assertThrows(AutoConfig.InvalidTokenException.class, () -> AutoConfig.decodeToken(token));
    }
}
