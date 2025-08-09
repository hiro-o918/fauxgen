from typing import Annotated

import pandas as pd
import pandera as pa
from pandera.typing import Series


class TestSchema(pa.DataFrameModel):
    int_col: Series[int] = pa.Field()
    pa_int_col: Series[pa.Int] = pa.Field()
    pa_int8_col: Series[pa.Int8] = pa.Field()
    pa_int16_col: Series[pa.Int16] = pa.Field()
    pa_int32_col: Series[pa.Int32] = pa.Field()
    pa_int64_col: Series[pa.Int64] = pa.Field()
    float_col: Series[float] = pa.Field()
    pa_float_col: Series[pa.Float] = pa.Field()
    pa_float16_col: Series[pa.Float16] = pa.Field()
    pa_float32_col: Series[pa.Float32] = pa.Field()
    pa_float64_col: Series[pa.Float64] = pa.Field()
    pa_float128_col: Series[pa.Float128] = pa.Field()
    str_col: Series[str] = pa.Field()
    pa_str_col: Series[pa.String] = pa.Field()
    bool_col: Series[bool] = pa.Field()
    pa_bool_col: Series[pa.Bool] = pa.Field()
    pa_datetime_col: Series[pa.DateTime] = pa.Field()
    pa_datetime_with_tz_col: Series[Annotated[pd.DatetimeTZDtype, "UTC"]] = pa.Field(nullable=True, coerce=True, description="Datetime with timezone in UTC")
    pa_date_col: Series[pa.Date] = pa.Field()
