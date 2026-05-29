package com.svix.kotlin

import com.svix.kotlin.models.EndpointIn
import java.util.Base64
import kotlin.test.Test
import kotlin.test.assertFailsWith

class AutoConfigTest {
    private fun minimalEndpoint(): EndpointIn = EndpointIn(url = "https://hook.example.test")

    private fun encodeToken(json: String): String =
        AUTOCONFIG_TOKEN_PREFIX_V1 +
            Base64.getEncoder().encodeToString(json.toByteArray(Charsets.UTF_8))

    @Test
    fun validTokenDoesNotThrow() {
        val json =
            """{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"""
        val token = encodeToken(json)

        AutoConfig(token, minimalEndpoint())
    }

    @Test
    fun badPrefixThrowsInvalidAutoConfigTokenException() {
        val json =
            """{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}"""
        val token = "wrong_" + Base64.getEncoder().encodeToString(json.toByteArray(Charsets.UTF_8))

        assertFailsWith<InvalidAutoConfigTokenException> { AutoConfig(token, minimalEndpoint()) }
    }

    @Test
    fun invalidJsonThrowsInvalidAutoConfigTokenException() {
        val token =
            AUTOCONFIG_TOKEN_PREFIX_V1 +
                Base64.getEncoder().encodeToString("not json".toByteArray(Charsets.UTF_8))

        assertFailsWith<InvalidAutoConfigTokenException> { AutoConfig(token, minimalEndpoint()) }
    }

    companion object {
        private const val AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"
    }
}
