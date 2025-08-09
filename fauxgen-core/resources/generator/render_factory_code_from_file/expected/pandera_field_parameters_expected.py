import datetime
from typing import Any, TypedDict

import fauxgen as f


class TestSchemaRecord(TypedDict):
    """A data structure representing TestSchema entries."""
    int_col: int
    float_col_only_ge: float
    float_col_only_le: float
    pa_bool_col: bool | None


def test_schema_record(
    *,
    int_col: int | f.Unset = f.UNSET,
    float_col_only_ge: float | f.Unset = f.UNSET,
    float_col_only_le: float | f.Unset = f.UNSET,
    pa_bool_col: bool | None | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> TestSchemaRecord:
    """Creates a mock TestSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        int_col (int): An integer column with limits
        float_col_only_ge (float): A float column with lower limit
        float_col_only_le (float): A float column with upper limit
        pa_bool_col (bool | None): A nullable boolean column
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        TestSchemaRecord: A new mock entry with generated data.
    """
    return {
        "int_col": f.Unset.unwrap_or_else(int_col, lambda: f.gen_int(ge=0.1, le=10, seed=seed_)),
        "float_col_only_ge": f.Unset.unwrap_or_else(float_col_only_ge, lambda: f.gen_float(ge=10, le=110, seed=seed_)),
        "float_col_only_le": f.Unset.unwrap_or_else(float_col_only_le, lambda: f.gen_float(ge=-90, le=10, seed=seed_)),
        "pa_bool_col": f.Unset.unwrap_or_else(pa_bool_col, lambda: f.gen_bool(seed=seed_)),
    }
