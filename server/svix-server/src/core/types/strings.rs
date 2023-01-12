#[macro_export]
macro_rules! string_wrapper {
    ($name_id:ident) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name_id(pub String);

        impl Deref for $name_id {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name_id> for sea_orm::Value {
            fn from(v: $name_id) -> Self {
                Self::String(Some(Box::new(v.0)))
            }
        }

        impl sea_orm::TryGetable for $name_id {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                match String::try_get(res, pre, col) {
                    Ok(v) => Ok($name_id(v)),
                    Err(e) => Err(e),
                }
            }
        }

        impl sea_orm::sea_query::Nullable for $name_id {
            fn null() -> sea_orm::Value {
                sea_orm::Value::String(None)
            }
        }

        impl sea_orm::sea_query::ValueType for $name_id {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::String(Some(x)) => Ok($name_id(*x)),
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                String::column_type()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                String::array_type()
            }
        }

        impl std::fmt::Display for $name_id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
