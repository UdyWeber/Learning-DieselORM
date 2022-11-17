// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Int4,
        name -> Varchar,
        email -> Varchar,
        maried -> Nullable<Bool>,
        removed -> Bool,
        gender -> Text,
    }
}
