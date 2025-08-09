from ...base_model import BaseModel
from ..child_model import ChildModel
import pandera as pa
from pandera.typing import Series


# 相対インポートで2段階上のベースモデルを継承
class GrandchildModel(BaseModel):
    gender: Series[str] = pa.Field(isin=["male", "female", "other"])
    score: Series[float] = pa.Field(ge=0.0, le=100.0)

    class Config:
        name = "grandchild_model"


# 相対インポートで1段階上の子モデルを継承
class NestedChildModel(ChildModel):
    status: Series[str] = pa.Field(isin=["active", "inactive", "pending"])
    last_login: Series[str] = pa.Field(nullable=True)

    class Config:
        name = "nested_child_model"
