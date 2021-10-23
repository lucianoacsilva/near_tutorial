use super::method::HTTP_method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: HTTP_method
        }