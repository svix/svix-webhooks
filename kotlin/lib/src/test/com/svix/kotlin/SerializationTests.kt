package com.svix.kotlin

import com.svix.kotlin.models.CronConfig
import com.svix.kotlin.models.IngestSourceIn
import com.svix.kotlin.models.IngestSourceInConfig
import com.svix.kotlin.models.IngestSourceInConfig.Cron
import kotlin.test.assertEquals
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Test
import kotlin.test.assertIs

class SerializationTests {
    @Test
    fun structEnumWithFields() {
        val jsonSource =
            """{"name":"name","uid":"uuiidd","type":"cron","config":{"contentType":"web-app/json","payload":"totally json fr fr","schedule":"* * * * *"}}"""
        val sourceIn =
            IngestSourceIn(
                name = "name",
                uid = "uuiidd",
                config = Cron(
                    CronConfig(
                        contentType = "web-app/json",
                        payload = "totally json fr fr",
                        schedule = "* * * * *",
                    )
                ),
            )
        // de/serialization works both ways
        assertEquals(jsonSource, Json.encodeToString(sourceIn))
        assertEquals(sourceIn, Json.decodeFromString(jsonSource))
    }

    @Test
    fun structEnumWithNoExtraFields() {
        val jsonSource =
            """{"name":"mendy is","uid":"very unique","type":"generic-webhook","config":{}}"""
        val sourceIn =
            IngestSourceIn(
                name = "mendy is",
                uid = "very unique",
                config = IngestSourceInConfig.GenericWebhook,
            )
        // de/serialization works both ways
        assertEquals(jsonSource, Json.encodeToString(sourceIn))
        assertEquals(sourceIn, Json.decodeFromString(jsonSource))
    }

    @Test
    fun readStructEnumField() {
        val jsonSource =
            """{"name":"name","uid":"uuiidd","type":"cron","config":{"contentType":"web-app/json","payload":"totally json fr fr","schedule":"* * * * *"}}"""
        val sourceIn = Json.decodeFromString(IngestSourceIn.serializer(), jsonSource)

        assertIs<IngestSourceInConfig.Cron>(sourceIn.config)
        // the assertIs smart casted for us
        assertEquals("* * * * *", sourceIn.config.cron.schedule)
    }
}