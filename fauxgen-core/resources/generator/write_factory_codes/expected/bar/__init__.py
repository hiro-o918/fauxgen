import datetime
from typing import Any, TypedDict

import fauxgen as f


class BarSchemaRecord(TypedDict):
    """A data structure representing BarSchema entries."""
    bar: int


def bar_schema_record(
    *,
    bar: int | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> BarSchemaRecord:
    """Creates a mock BarSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        bar (int): Field bar
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        BarSchemaRecord: A new mock entry with generated data.
    """
    return {
        "bar": f.Unset.unwrap_or_else(bar, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
    }
