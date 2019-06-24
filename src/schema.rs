table! {
    files (id) {
        id -> Int4,
        name -> Varchar,
        file_name -> Varchar,
        folder_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        extension -> Varchar,
    }
}

table! {
    folders (id) {
        id -> Int4,
        name -> Varchar,
        parent_id -> Nullable<Int4>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        root -> Nullable<Int4>,
        role -> Varchar,
    }
}

joinable!(files -> folders (folder_id));

allow_tables_to_appear_in_same_query!(files, folders, users,);
