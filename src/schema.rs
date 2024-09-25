// @generated automatically by Diesel CLI.

diesel::table! {
    students (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 20]
        email -> Varchar,
        #[max_length = 20]
        password -> Varchar,
    }
}
