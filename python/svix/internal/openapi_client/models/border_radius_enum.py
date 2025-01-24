from enum import Enum


class BorderRadiusEnum(str, Enum):
    FULL = "full"
    LG = "lg"
    MD = "md"
    NONE = "none"
    SM = "sm"

    def __str__(self) -> str:
        return str(self.value)
