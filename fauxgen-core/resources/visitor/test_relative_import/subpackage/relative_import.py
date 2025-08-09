import pandera as pa
from pandas import Series

# 相対インポートを使用
from ..base_model import BaseModel


class RelativeUser(BaseModel):
    """User model that extends a model imported using relative import."""
    name: Series[pa.String] = pa.Field(description="User's name")
    age: Series[pa.Int] = pa.Field(ge=0, description="User's age")