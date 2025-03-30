import datetime
from typing import TypedDict

import fauxgen as f


class FooSchemaRecord(TypedDict):
    """A data structure representing FooSchema entries."""
    foo: int


def foo_schema_record(
    *,
    foo: int | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> FooSchemaRecord:
    """Creates a mock FooSchema entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        foo (int): Field foo
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        FooSchemaRecord: A new mock entry with generated data.
    """
    return {
        "foo": f.Unset.unwrap_or_else(foo, lambda: f.gen_int(ge=0, le=100, seed=seed_)),
    }
