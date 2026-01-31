// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 70]
        personal_email -> Varchar,
        salt -> Bytea,
        password_mac -> Bytea,
        password_sha1 -> Bytea,
    }
}
