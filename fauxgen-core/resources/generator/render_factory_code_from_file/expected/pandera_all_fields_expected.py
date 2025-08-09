import datetime
from typing import Any, TypedDict

import fauxgen as f


class TestSchemaRecord(TypedDict):
    """A data structure representing TestSchema entries."""
    int_col: int
    pa_int_col: int
    pa_int8_col: int
    pa_int16_col: int
    pa_int32_col: int
    pa_int64_col: int
    float_col: float
    pa_float_col: float
    pa_float16_col: float
    pa_float32_col: float
    pa_float64_col: float
    pa_float128_col: float
    str_col: str
    pa_str_col: str
    bool_col: bool
    pa_bool_col: bool
    pa_datetime_col: datetime.datetime
    pa_datetime_with_tz_col: datetime.datetime | None
    pa_date_col: datetime.date


def test_schema_record(
    *,
    int_col: int | f.Unset = f.UNSET,
    pa_int_col: int | f.Unset = f.UNSET,
    pa_int8_col: int | f.Unset = f.UNSET,
    pa_int16_col: int | f.Unset = f.UNSET,
    pa_int32_col: int | f.Unset = f.UNSET,
    pa_int64_col: int | f.Unset = f.UNSET,
    float_col: float | f.Unset = f.UNSET,
    pa_float_col: float | f.Unset = f.UNSET,
    pa_float16_col: float | f.Unset = f.UNSET,
    pa_float32_col: float | f.Unset = f.UNSET,
    pa_float64_col: float | f.Unset = f.UNSET,
    pa_float128_col: float | f.Unset = f.UNSET,
    str_col: str | f.Unset = f.UNSET,
    pa_str_col: str | f.Unset = f.UNSET,
    bool_col: bool | f.Unset = f.UNSET,
    pa_bool_col: bool | f.Unset = f.UNSET,
    pa_datetime_col: datetime.datetime | f.Unset = f.UNSET,
    pa_datetime_with_tz_col: datetime.datetime | None | f.Unset = f.UNSET,
    pa_date_col: datetime.date | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> TestSchemaRecord:
    """Creates a mock TestSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        int_col (int): Field int_col
        pa_int_col (int): Field pa_int_col
        pa_int8_col (int): Field pa_int8_col
        pa_int16_col (int): Field pa_int16_col
        pa_int32_col (int): Field pa_int32_col
        pa_int64_col (int): Field pa_int64_col
        float_col (float): Field float_col
        pa_float_col (float): Field pa_float_col
        pa_float16_col (float): Field pa_float16_col
        pa_float32_col (float): Field pa_float32_col
        pa_float64_col (float): Field pa_float64_col
        pa_float128_col (float): Field pa_float128_col
        str_col (str): Field str_col
        pa_str_col (str): Field pa_str_col
        bool_col (bool): Field bool_col
        pa_bool_col (bool): Field pa_bool_col
        pa_datetime_col (datetime.datetime): Field pa_datetime_col
        pa_datetime_with_tz_col (datetime.datetime | None): Datetime with timezone in UTC
        pa_date_col (datetime.date): Field pa_date_col
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        TestSchemaRecord: A new mock entry with generated data.
    """
    return {
        "int_col": f.Unset.unwrap_or_else(int_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int_col": f.Unset.unwrap_or_else(pa_int_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int8_col": f.Unset.unwrap_or_else(pa_int8_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int16_col": f.Unset.unwrap_or_else(pa_int16_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int32_col": f.Unset.unwrap_or_else(pa_int32_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "pa_int64_col": f.Unset.unwrap_or_else(pa_int64_col, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "float_col": f.Unset.unwrap_or_else(float_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float_col": f.Unset.unwrap_or_else(pa_float_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float16_col": f.Unset.unwrap_or_else(pa_float16_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float32_col": f.Unset.unwrap_or_else(pa_float32_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float64_col": f.Unset.unwrap_or_else(pa_float64_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "pa_float128_col": f.Unset.unwrap_or_else(pa_float128_col, lambda: f.gen_float(ge=0, le=100, seed=seed_)),
        "str_col": f.Unset.unwrap_or_else(str_col, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "pa_str_col": f.Unset.unwrap_or_else(pa_str_col, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "bool_col": f.Unset.unwrap_or_else(bool_col, lambda: f.gen_bool(seed=seed_)),
        "pa_bool_col": f.Unset.unwrap_or_else(pa_bool_col, lambda: f.gen_bool(seed=seed_)),
        "pa_datetime_col": f.Unset.unwrap_or_else(pa_datetime_col, lambda: f.gen_datetime(from_datetime=datetime.datetime(2020, 1, 1), to_datetime=datetime.datetime(2021, 1, 1), seed=seed_)),
        "pa_datetime_with_tz_col": f.Unset.unwrap_or_else(pa_datetime_with_tz_col, lambda: f.gen_datetime(from_datetime=datetime.datetime(2020, 1, 1), to_datetime=datetime.datetime(2021, 1, 1), seed=seed_)),
        "pa_date_col": f.Unset.unwrap_or_else(pa_date_col, lambda: f.gen_date(from_date=datetime.date(2020, 1, 1), to_date=datetime.date(2021, 1, 1), seed=seed_)),
    }
