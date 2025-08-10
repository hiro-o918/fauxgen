import pandera.polars as pa
from pandera.typing import Series

class PolarsModel(pa.DataFrameModel):
    """Base model for Polars DataFrame with common fields."""

    id: Series[int] = pa.Field(ge=1, description="Unique identifier")
    created_at: Series[pa.DateTime] = pa.Field(description="Creation timestamp")
