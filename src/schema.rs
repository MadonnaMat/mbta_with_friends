table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        friend_id -> Int4,
    }
}

table! {
    lines (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    stops (id) {
        id -> Int4,
        name -> Varchar,
        api_id -> Varchar,
        longitude -> Float8,
        latitude -> Float8,
    }
}

table! {
    sublines (id) {
        id -> Int4,
        line_id -> Int4,
        name -> Varchar,
        api_id -> Varchar,
    }
}

table! {
    subline_stops (id) {
        id -> Int4,
        subline_id -> Int4,
        stop_id -> Int4,
        stop_sequence -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(subline_stops -> stops (stop_id));
joinable!(subline_stops -> sublines (subline_id));
joinable!(sublines -> lines (line_id));

allow_tables_to_appear_in_same_query!(
    friends,
    lines,
    stops,
    sublines,
    subline_stops,
    users,
);
