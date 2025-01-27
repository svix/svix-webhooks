from pydantic import BaseModel, ConfigDict
from re import sub


def to_camelcase(s: str) -> str:
    s = sub(r"(_|-)+", " ", s).title().replace(" ", "").replace("*", "")
    return "".join([s[0].lower(), s[1:]])


class SvixBaseModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camelcase,
        populate_by_name=True,
        json_encoders={set: lambda v: list(v)},
    )
