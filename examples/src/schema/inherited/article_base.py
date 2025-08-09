import pandera as pa
from pandera.typing import Series


class ArticleBase(pa.DataFrameModel):
    """Base article schema with common fields"""
    id: Series[int] = pa.Field(ge=1, description="Unique identifier for the article")
    title: Series[str] = pa.Field(nullable=False, description="Article title")
    author_id: Series[int] = pa.Field(ge=1, description="Unique identifier of the article's author")


class BlogArticle(ArticleBase):
    """Blog article schema that inherits from base article"""
    content: Series[str] = pa.Field(nullable=True, description="Main body of the blog article")
    published_at: Series[pa.DateTime] = pa.Field(nullable=True, description="Timestamp when the article was published")
    is_published: Series[bool] = pa.Field(description="Flag indicating whether the blog is publicly available")
    tags: Series[str] = pa.Field(nullable=True, description="Comma-separated tags for the blog")