import datetime
from typing import Any, TypedDict

import fauxgen as f


class PandasAliasSchemaRecord(TypedDict):
    """A data structure representing PandasAliasSchema entries."""
    user_id: int
    name: str
    active: bool


def pandas_alias_schema_record(
    *,
    user_id: int | f.Unset = f.UNSET,
    name: str | f.Unset = f.UNSET,
    active: bool | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> PandasAliasSchemaRecord:
    """Creates a mock PandasAliasSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        user_id (int): Field user_id
        name (str): Field name
        active (bool): Field active
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        PandasAliasSchemaRecord: A new mock entry with generated data.
    """
    return {
        "user_id": f.Unset.unwrap_or_else(user_id, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "name": f.Unset.unwrap_or_else(name, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "active": f.Unset.unwrap_or_else(active, lambda: f.gen_bool(seed=seed_)),
    }
