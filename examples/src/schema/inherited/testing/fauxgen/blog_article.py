import datetime
from typing import Any, TypedDict

import fauxgen as f


class BlogArticleRecord(TypedDict):
    """A data structure representing BlogArticle entries."""
    id: int
    title: str
    author_id: int


def blog_article_record(
    *,
    id: int | f.Unset = f.UNSET,
    title: str | f.Unset = f.UNSET,
    author_id: int | f.Unset = f.UNSET,
    seed_: int | None = None,
) -> BlogArticleRecord:
    """Creates a mock BlogArticle entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
        id (int): Unique identifier for the article
        title (str): Article title
        author_id (int): Unique identifier of the article's author
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        BlogArticleRecord: A new mock entry with generated data.
    """
    return {
        "id": f.Unset.unwrap_or_else(id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
        "title": f.Unset.unwrap_or_else(title, lambda: f.gen_string(min_length=5, max_length=10, seed=seed_)),
        "author_id": f.Unset.unwrap_or_else(author_id, lambda: f.gen_int(ge=1, le=101, seed=seed_)),
    }
