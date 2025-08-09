import pandera as pa
from pandas import Series

# Import with aliases to test alias resolution
from visitor.base_models import BaseDataFrameModel as BaseModel


class AliasUser(BaseModel):
    """User model that extends an aliased import."""
    name: Series[pa.String] = pa.Field(description="User's name")
    preferences: Series[pa.String] = pa.Field(description="User preferences")
