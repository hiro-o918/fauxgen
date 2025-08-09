import pandera as pa
from pandera.typing import Series

from .article_base import ArticleBase


class NewsArticle(ArticleBase):
    """News article schema that inherits from base article"""
    content: Series[str] = pa.Field(nullable=False, description="Main body of the news article")
    published_at: Series[pa.DateTime] = pa.Field(nullable=False, description="Timestamp when the news was published")
    category: Series[str] = pa.Field(description="News category")
    source: Series[str] = pa.Field(nullable=True, description="Source of the news")