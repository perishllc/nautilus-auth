// @generated automatically by Diesel CLI.

diesel::table! {
    seeds (identifier) {
        identifier -> Varchar,
        encrypted_seed -> Varchar,
    }
}
