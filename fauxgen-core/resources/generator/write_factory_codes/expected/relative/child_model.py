import datetime
from typing import Any, TypedDict, Optional

import fauxgen as f


class ChildModelRecord(TypedDict):
    """A data structure representing ChildModel entries."""
    id: int
    name: str
    age: int
    email: Optional[str]


def child_model_record(
    *,
    id: int | f.Unset = f.UNSET,
    name: str | f.Unset = f.UNSET,
    age: int | f.Unset = f.UNSET,
    email: Optional[str] | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> ChildModelRecord:
    """Creates a mock ChildModel entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        id (int): Field id
        name (str): Field name
        age (int): Field age
        email (Optional[str]): Field email
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        ChildModelRecord: A new mock entry with generated data.
    """
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
        "name": f.Unset.unwrap_or_else(name, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "age": f.Unset.unwrap_or_else(age, lambda: f.gen_int(ge=0, le=120, seed=seed_)),
        "email": f.Unset.unwrap_or_else(email, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
    }
