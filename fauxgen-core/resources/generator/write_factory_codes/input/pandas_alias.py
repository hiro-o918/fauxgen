import pandera as pa
from pandera.typing import Series


class PandasAliasSchema(pa.DataFrameModel):
    user_id: Series[int] = pa.Field(gt=0)
    name: Series[str] = pa.Field(str_length={"min_value": 3, "max_value": 50})
    active: Series[bool] = pa.Field()
