import datetime
import pandera.pandas as pa
from pandera.typing import Series

class PandasModel(pa.DataFrameModel):
    """Base model for Pandas DataFrame with common fields."""

    id: Series[int] = pa.Field(ge=1, description="Unique identifier")
    created_at: Series[pa.DateTime] = pa.Field(description="Creation timestamp")
