from pydantic import BaseModel, ConfigDict


class SvixBaseModel(BaseModel):
    model_config = ConfigDict(
        populate_by_name=True,
        json_encoders={set: lambda v: list(v)},
    )
