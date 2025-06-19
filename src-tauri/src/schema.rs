// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Nullable<Integer>,
        name -> Text,
        path -> Text,
        project_id -> Integer,
        user_id -> Integer,
        template_type_id -> Nullable<Integer>,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        project_id -> Integer,
        title -> Text,
        content -> Nullable<Text>,
        content_format -> Text,
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
        path -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    templates_types (template_id, type_id) {
        template_id -> Integer,
        type_id -> Integer,
    }
}

diesel::table! {
    types (id) {
        id -> Nullable<Integer>,
        name -> Text,
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

diesel::joinable!(files -> projects (project_id));
diesel::joinable!(files -> users (user_id));
diesel::joinable!(notes -> projects (project_id));
diesel::joinable!(projects -> users (user_id));
diesel::joinable!(templates_types -> templates (template_id));
diesel::joinable!(templates_types -> types (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    notes,
    projects,
    templates,
    templates_types,
    types,
    user_representatives,
    users,
);
