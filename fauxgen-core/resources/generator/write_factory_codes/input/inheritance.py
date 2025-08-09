import pandera as pa
from pandera.typing import Series


# ベースモデルの定義
class BaseSchema(pa.DataFrameModel):
    id: Series[int] = pa.Field(gt=0)
    created_at: Series[str] = pa.Field()

    class Config:
        name = "base_schema"


# 継承したモデルの定義
class UserSchema(BaseSchema):
    username: Series[str] = pa.Field(str_length={"min_value": 3, "max_value": 20})
    email: Series[str] = pa.Field(
        regex=r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    )

    class Config:
        name = "user_schema"


# さらに継承したモデル
class AdminSchema(UserSchema):
    admin_level: Series[int] = pa.Field(ge=1, le=5)
    department: Series[str] = pa.Field(isin=["IT", "HR", "Finance", "Operations"])

    class Config:
        name = "admin_schema"
