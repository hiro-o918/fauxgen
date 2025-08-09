import pandera as pa
from pandas import Series


class BaseModel(pa.DataFrameModel):
    """Base model that defines common fields for all models."""

    id: Series[pa.Int] = pa.Field(ge=1, description="Unique identifier")
    created_at: Series[pa.DateTime] = pa.Field(description="Creation timestamp")
