import datetime
from typing import Any, TypedDict

import fauxgen as f


class UserSchemaRecord(TypedDict):
    """A data structure representing UserSchema entries."""
    id: int
    age: int
    name: str
    email: str | None
    active: bool


def user_schema_record(
    *,
    id: int | f.Unset = f.UNSET,
    age: int | f.Unset = f.UNSET,
    name: str | f.Unset = f.UNSET,
    email: str | None | f.Unset = f.UNSET,
    active: bool | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> UserSchemaRecord:
    """Creates a mock UserSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        id (int): Unique identifier for the user
        age (int): User's age in years
        name (str): User's full name
        email (str | None): User's contact email address
        active (bool): Flag indicating whether the user account is enabled
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        UserSchemaRecord: A new mock entry with generated data.
    """
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
        "age": f.Unset.unwrap_or_else(age, lambda: f.gen_int(ge=0, le=150, seed=seed_)),
        "name": f.Unset.unwrap_or_else(name, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "email": f.Unset.unwrap_or_else(email, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "active": f.Unset.unwrap_or_else(active, lambda: f.gen_bool(seed=seed_)),
    }
