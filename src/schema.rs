table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
