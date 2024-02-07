pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for String {
    fn to_snake_case(&self) -> String {
        self.to_string().to_lowercase().replace(" ", "_")
    }
}
