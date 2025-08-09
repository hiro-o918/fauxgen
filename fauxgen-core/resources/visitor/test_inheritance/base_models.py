import pandera as pa
from pandas import Series


class BaseDataFrameModel(pa.DataFrameModel):
    """Base model that defines common fields for all models."""

    id: Series[pa.Int] = pa.Field(ge=1, description="Unique identifier")
    created_at: Series[pa.DateTime] = pa.Field(description="Creation timestamp")

    class Config:
        strict = True


class UserBase(BaseDataFrameModel):
    """Base user model with common user fields."""

    username: Series[pa.String] = pa.Field(description="User's username")
    email: Series[pa.String] = pa.Field(description="User's email address")
