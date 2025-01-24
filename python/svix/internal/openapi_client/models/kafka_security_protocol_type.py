from enum import Enum


class KafkaSecurityProtocolType(str, Enum):
    PLAINTEXT = "plaintext"
    SASL_SSL = "sasl-ssl"
    SSL = "ssl"

    def __str__(self) -> str:
        return str(self.value)
