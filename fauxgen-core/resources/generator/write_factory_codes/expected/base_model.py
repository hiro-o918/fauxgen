import datetime
from typing import Any, TypedDict

import fauxgen as f


class BaseModelRecord(TypedDict):
    """A data structure representing BaseModel entries."""
    id: int
    name: str


def base_model_record(
    *,
    id: int | f.Unset = f.UNSET,
    name: str | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> BaseModelRecord:
    """Creates a mock BaseModel entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        id (int): Field id
        name (str): Field name
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        BaseModelRecord: A new mock entry with generated data.
    """
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "name": f.Unset.unwrap_or_else(name, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
    }
