from ...base_model import BaseModel
from ..child_model import ChildModel
import pandera as pa
from pandera.typing import Series


# load relative import of base model from two levels up
class GrandchildModel(BaseModel):
    gender: Series[str] = pa.Field(isin=["male", "female", "other"])
    score: Series[float] = pa.Field(ge=0.0, le=100.0)

    class Config:
        name = "grandchild_model"


# load relative import of child model from one level up
class NestedChildModel(ChildModel):
    status: Series[str] = pa.Field(isin=["active", "inactive", "pending"])
    last_login: Series[str] = pa.Field(nullable=True)

    class Config:
        name = "nested_child_model"
