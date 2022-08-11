table! {
    group_users (id) {
        id -> Int4,
        group_id -> Int4,
        member_id -> Int4,
    }
}

table! {
    groups (id) {
        id -> Int4,
        owner_id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(group_users -> groups (group_id));
joinable!(group_users -> users (member_id));

allow_tables_to_appear_in_same_query!(
    group_users,
    groups,
    users,
);
