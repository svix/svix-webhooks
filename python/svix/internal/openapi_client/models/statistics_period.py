from enum import Enum


class StatisticsPeriod(str, Enum):
    FIVEMINUTES = "FiveMinutes"
    ONEDAY = "OneDay"

    def __str__(self) -> str:
        return str(self.value)
