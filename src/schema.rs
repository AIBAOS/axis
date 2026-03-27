diesel::table! {
    audit_logs (id) {
        id -> Integer,
        user_id -> Integer,
        action -> Text,
        resource -> Text,
        details -> Nullable<Text>,
        ip_address -> Text,
        created_at -> Timestamp,
    }
}