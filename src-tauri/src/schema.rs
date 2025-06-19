// @generated automatically by Diesel CLI.

diesel::table! {
    file_formats (id) {
        id -> Nullable<Integer>,
        name -> Text,
        extension -> Text,
        mime_type -> Nullable<Text>,
        editable -> Nullable<Bool>,
        previewable -> Nullable<Bool>,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    files (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        file_format_id -> Integer,
        relative_path -> Text,
        content -> Nullable<Text>,
        is_template -> Nullable<Bool>,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    project_files (id) {
        id -> Nullable<Integer>,
        project_id -> Integer,
        file_id -> Integer,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        path -> Text,
        user_id -> Integer,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    templates (id) {
        id -> Nullable<Integer>,
        file_id -> Integer,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    types_templates (id) {
        id -> Nullable<Integer>,
        name -> Text,
        alias -> Text,
        description -> Nullable<Text>,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_representatives (user_id, representative_id) {
        user_id -> Integer,
        representative_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        last_name -> Text,
        main_path -> Nullable<Text>,
        user_type -> Text,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(files -> file_formats (file_format_id));
diesel::joinable!(project_files -> files (file_id));
diesel::joinable!(project_files -> projects (project_id));
diesel::joinable!(projects -> users (user_id));
diesel::joinable!(templates -> files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_formats,
    files,
    project_files,
    projects,
    templates,
    types_templates,
    user_representatives,
    users,
);
