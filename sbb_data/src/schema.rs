table! {
    past_dailies (id) {
        id -> Int8,
        robot_id -> Int8,
        posted_on -> Date,
    }
}

table! {
    reply_tweets (id) {
        id -> Int8,
        request_tweet_id -> Int8,
        request_tweet_time -> Timestamptz,
        reply_tweet_id -> Int8,
        reply_tweet_time -> Timestamptz,
        user_id -> Int8,
        user_handle -> Text,
        robot_id -> Int8,
    }
}

table! {
    robot_groups (id) {
        id -> Int8,
        tweet_id -> Int8,
        tweet_time -> Timestamptz,
        image_url -> Text,
        body -> Text,
        alt -> Nullable<Text>,
        content_warning -> Nullable<Text>,
    }
}

table! {
    robots (id) {
        id -> Int8,
        robot_group_id -> Int8,
        robot_number -> Int4,
        prefix -> Text,
        suffix -> Text,
        plural -> Nullable<Text>,
        ident -> Text,
    }
}

table! {
    scheduled_dailies (id) {
        id -> Int8,
        robot_id -> Int8,
        post_on -> Date,
    }
}

table! {
    tagged_markers (robot_group_id) {
        robot_group_id -> Int8,
        tagged_at -> Timestamptz,
    }
}

table! {
    tags (id) {
        id -> Int8,
        robot_group_id -> Int8,
        tag -> Text,
    }
}

joinable!(past_dailies -> robots (robot_id));
joinable!(reply_tweets -> robots (robot_id));
joinable!(robots -> robot_groups (robot_group_id));
joinable!(scheduled_dailies -> robots (robot_id));
joinable!(tagged_markers -> robot_groups (robot_group_id));
joinable!(tags -> robot_groups (robot_group_id));

allow_tables_to_appear_in_same_query!(
    past_dailies,
    reply_tweets,
    robot_groups,
    robots,
    scheduled_dailies,
    tagged_markers,
    tags,
);
