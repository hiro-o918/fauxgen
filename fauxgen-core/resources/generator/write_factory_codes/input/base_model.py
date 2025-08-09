import pandera as pa
from pandera.typing import Series


class BaseModel(pa.DataFrameModel):
    id: Series[int] = pa.Field(gt=0)
    name: Series[str] = pa.Field()
