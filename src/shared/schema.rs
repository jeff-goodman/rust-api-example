// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    users,
);
