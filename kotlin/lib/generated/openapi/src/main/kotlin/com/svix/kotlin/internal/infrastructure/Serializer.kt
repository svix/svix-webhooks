package com.svix.kotlin.internal.infrastructure

import com.squareup.moshi.FromJson
import com.squareup.moshi.Moshi
import com.squareup.moshi.ToJson
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import com.svix.kotlin.models.MessageAttemptTriggerType
import com.svix.kotlin.models.MessageStatus

object Serializer {
    @JvmStatic
    val moshiBuilder: Moshi.Builder = Moshi.Builder()
        .add(OffsetDateTimeAdapter())
        .add(LocalDateTimeAdapter())
        .add(LocalDateAdapter())
        .add(UUIDAdapter())
        .add(ByteArrayAdapter())
        .add(URIAdapter())
        .add(KotlinJsonAdapterFactory())
        .add(BigDecimalAdapter())
        .add(BigIntegerAdapter())
        .add(MessageStatusAdapter())
        .add(MessageAttemptTriggerType())

    @JvmStatic
    val moshi: Moshi by lazy {
        moshiBuilder.build()
    }
}

class MessageStatusAdapter {
    @ToJson
    fun toJson(messageStatus: MessageStatus): Int {
        return messageStatus.value
    }

    @FromJson
    fun fromJson(messageStatus: Int): MessageStatus {
        MessageStatus.values().forEach {
            if (it.value == messageStatus) {
                return it
            }
        }
        return MessageStatus.Unknown
    }
}

class MessageAttemptTriggerType {
    @ToJson
    fun toJson(triggerType: MessageAttemptTriggerType): Int {
        return triggerType.value
    }

    @FromJson
    fun fromJson(triggerType: Int): MessageAttemptTriggerType {
        MessageAttemptTriggerType.values().forEach {
            if (it.value == triggerType) {
                return it
            }
        }
        return MessageAttemptTriggerType.Unknown
    }
}
