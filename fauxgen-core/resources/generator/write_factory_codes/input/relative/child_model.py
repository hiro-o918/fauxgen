from ..base_model import BaseModel
from pandera.typing import Series
import pandera as pa


class ChildModel(BaseModel):
    age: Series[int] = pa.Field(ge=0, le=120)
    email: Series[str] = pa.Field(nullable=True)
