import pandera.pandas as pa
from pandera.typing import Series


class Article(pa.DataFrameModel):
    id: Series[int] = pa.Field(ge=1, description="Unique identifier for the article")
    title: Series[str] = pa.Field(nullable=False, description="Article title")
    content: Series[str] = pa.Field(nullable=True, description="Main body of the article")
    author_id: Series[int] = pa.Field(ge=1, description="Unique identifier of the article's author")
    published_at: Series[pa.DateTime] = pa.Field(nullable=True, description="Timestamp when the article was published")
    is_published: Series[bool] = pa.Field(description="Flag indicating whether the article is publicly available")
