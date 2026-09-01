package com.svix.kotlin

import com.svix.kotlin.models.EndpointIn
import java.util.Base64
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class AutoConfigTest {
    private fun minimalEndpoint(): EndpointIn = EndpointIn(url = "https://hook.example.test")

    private fun encodeToken(prefix: String, json: String): String =
        prefix + Base64.getEncoder().encodeToString(json.toByteArray(Charsets.UTF_8))

    @Test
    fun validTokenDoesNotThrow() {
        val json =
            """{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"""
        val token = encodeToken(AUTOCONFIG_TOKEN_PREFIX_V1, json)

        AutoConfig(token, minimalEndpoint())
    }

    @Test
    fun validV2TokenDoesNotThrow() {
        val json =
            """{"aid":"app_1","sid":"acfg_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"""
        val token = encodeToken(AUTOCONFIG_TOKEN_PREFIX_V2, json)

        AutoConfig(token, minimalEndpoint())
    }

    @Test
    fun badPrefixThrowsInvalidAutoConfigTokenException() {
        val json =
            """{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}"""
        val token = "wrong_" + Base64.getEncoder().encodeToString(json.toByteArray(Charsets.UTF_8))

        val ex =
            assertFailsWith<InvalidAutoConfigTokenException> { AutoConfig(token, minimalEndpoint()) }
        assertEquals(
            "Unsupported token version. You might need to update the Svix SDK to use this token",
            ex.message,
        )
    }

    @Test
    fun invalidJsonThrowsInvalidAutoConfigTokenException() {
        val token = encodeToken(AUTOCONFIG_TOKEN_PREFIX_V1, "not json")

        assertFailsWith<InvalidAutoConfigTokenException> { AutoConfig(token, minimalEndpoint()) }
    }

    @Test
    fun invalidV2JsonThrowsInvalidAutoConfigTokenException() {
        val token = encodeToken(AUTOCONFIG_TOKEN_PREFIX_V2, "not json")

        assertFailsWith<InvalidAutoConfigTokenException> { AutoConfig(token, minimalEndpoint()) }
    }

    companion object {
        private const val AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"
        private const val AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_"
    }
}
