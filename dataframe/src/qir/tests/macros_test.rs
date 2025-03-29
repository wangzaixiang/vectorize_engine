#[cfg(test)]
mod tests {
    use crate::qir::{Column, DataType, Table};

    #[test]
    fn test_table_macro() {
        let users = table! {
            name: "users",
            columns: [
                column! { name = "id", data_type = I64 },
                column! { name = "name", data_type = String },
                column! { name = "age", data_type = I32 },
            ],
        };

        assert_eq!(users.name, "users");
        assert_eq!(users.columns.len(), 3);

        assert_eq!(users.columns[0].name, "id");
        matches!(users.columns[0].data_type, DataType::I64);

        assert_eq!(users.columns[1].name, "name");
        matches!(users.columns[1].data_type, DataType::String);

        assert_eq!(users.columns[2].name, "age");
        matches!(users.columns[2].data_type, DataType::I32);
    }

    #[test]
    fn test_column_macro() {
        let col: Column = column! { name = "id", data_type = I64 };

        assert_eq!(col.name, "id");
        matches!(col.data_type, DataType::I64);
    }

}
