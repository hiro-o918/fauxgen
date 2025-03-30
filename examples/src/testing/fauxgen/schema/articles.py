import datetime
from typing import TypedDict

import fauxgen as f


class ArticleRecord(TypedDict):
    """A data structure representing Article entries."""
    id: int
    title: str
    content: str | None
    author_id: int
    published_at: datetime.datetime | None
    is_published: bool


def article_record(
    *,
    id: int | f.Unset = f.UNSET,
    title: str | f.Unset = f.UNSET,
    content: str | None | f.Unset = f.UNSET,
    author_id: int | f.Unset = f.UNSET,
    published_at: datetime.datetime | None | f.Unset = f.UNSET,
    is_published: bool | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> ArticleRecord:
    """Creates a mock Article entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        id (int): Unique identifier for the article
        title (str): Article title
        content (str | None): Main body of the article
        author_id (int): Unique identifier of the article's author
        published_at (datetime.datetime | None): Timestamp when the article was published
        is_published (bool): Flag indicating whether the article is publicly available
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        ArticleRecord: A new mock entry with generated data.
    """
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
        "title": f.Unset.unwrap_or_else(title, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "content": f.Unset.unwrap_or_else(content, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "author_id": f.Unset.unwrap_or_else(author_id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
        "published_at": f.Unset.unwrap_or_else(published_at, lambda: f.gen_datetime(from_datetime=datetime.datetime(2020, 1, 1), to_datetime=datetime.datetime(2021, 1, 1), seed=seed_)),
        "is_published": f.Unset.unwrap_or_else(is_published, lambda: f.gen_bool(seed=seed_)),
    }
