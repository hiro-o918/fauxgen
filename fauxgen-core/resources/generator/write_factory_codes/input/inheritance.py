import pandera as pa
from pandera.typing import Series


class BaseSchema(pa.DataFrameModel):
    id: Series[int] = pa.Field(gt=0)
    created_at: Series[str] = pa.Field()


class UserSchema(BaseSchema):
    username: Series[str] = pa.Field(str_length={"min_value": 3, "max_value": 20})
    email: Series[str] = pa.Field(
        regex=r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    )

class AdminSchema(UserSchema):
    admin_level: Series[int] = pa.Field(ge=1, le=5)
    department: Series[str] = pa.Field(isin=["IT", "HR", "Finance", "Operations"])
