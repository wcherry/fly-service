// @generated automatically by Diesel CLI.

diesel::table! {
    file_folders (id) {
        id -> Text,
        owner_id -> Integer,
        parent_folder_id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}

diesel::table! {
    files (id) {
        id -> Text,
        owner_id -> Integer,
        access_level -> Integer,
        title -> Text,
        folder_id -> Text,
        media_type -> Nullable<Text>,
        orginal_filename -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email_address -> Text,
        password -> Text,
        folder_id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    file_folders,
    files,
    users,
);
