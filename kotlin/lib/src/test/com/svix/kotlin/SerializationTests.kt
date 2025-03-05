// this test depends on new models that are not yet merged (IngestSourceIn)
// those models come from a new openapi.json what is not yet merged
// so for now this test accomplishes nothing :(


//package com.svix.kotlin
//
//import com.svix.kotlin.models.CronConfig
//import com.svix.kotlin.models.IngestSourceIn
//import com.svix.kotlin.models.IngestSourceInType
//import kotlin.test.assertEquals
//import kotlinx.serialization.json.Json
//import org.junit.jupiter.api.Test
//
//class SerializationTests {
//    @Test
//    fun structEnumWithFields() {
//        val jsonSource =
//            """{"name":"name","uid":"uuiidd","type":"cron","config":{"contentType":"web-app/json","payload":"totally json fr fr","schedule":"* * * * *"}}"""
//        val sourceIn =
//            IngestSourceIn(
//                name = "name",
//                uid = "uuiidd",
//                type =
//                    IngestSourceInType.Cron(
//                        CronConfig(
//                            contentType = "web-app/json",
//                            payload = "totally json fr fr",
//                            schedule = "* * * * *",
//                        )
//                    ),
//            )
//        // de/serialization works both ways
//        assertEquals(jsonSource, Json.encodeToString(sourceIn))
//        assertEquals(sourceIn, Json.decodeFromString(jsonSource))
//    }
//
//    @Test
//    fun structEnumWithNoExtraFields() {
//        val jsonSource =
//            """{"name":"mendy is","uid":"very unique","type":"generic-webhook","config":{}}"""
//        val sourceIn =
//            IngestSourceIn(
//                name = "mendy is",
//                uid = "very unique",
//                type = IngestSourceInType.GenericWebhook,
//            )
//        // de/serialization works both ways
//        assertEquals(jsonSource, Json.encodeToString(sourceIn))
//        assertEquals(sourceIn, Json.decodeFromString(jsonSource))
//    }
//}
