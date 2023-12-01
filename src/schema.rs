table! {
    articles (id) {
        id -> Int8,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        tag_list -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        favorites_count -> Int4,
    }
}

table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        article -> Int8,
        author -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    favorites (user, article) {
        user -> Int4,
        article -> Int8,
    }
}

table! {
    follows (follower, followed) {
        follower -> Int4,
        followed -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(articles -> users (author));
joinable!(comments -> articles (article));
joinable!(comments -> users (author));
joinable!(favorites -> articles (article));
joinable!(favorites -> users (user));

allow_tables_to_appear_in_same_query!(articles, comments, favorites, follows, users,);
