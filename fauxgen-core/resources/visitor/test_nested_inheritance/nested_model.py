import pandera as pa
from pandas import Series


class BaseDataFrameModel(pa.DataFrameModel):
    """Base model that defines common fields for all models."""

    id: Series[pa.Int] = pa.Field(ge=1, description="Unique identifier")
    created_at: Series[pa.DateTime] = pa.Field(description="Creation timestamp")


class User(BaseDataFrameModel):
    """Model in a nested directory to test import resolution."""

    name: Series[pa.String] = pa.Field(description="Name field")
    value: Series[pa.Float] = pa.Field(ge=0.0, description="Value field")


class UserExtension(User):
    """Extension of a model from another module."""

    extra_field: Series[pa.String] = pa.Field(description="Additional user field")
    score: Series[pa.Float] = pa.Field(ge=0.0, le=100.0, description="User score")
