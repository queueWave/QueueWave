CREATE TABLE IF NOT EXISTS combined_message (
    id SERIAL PRIMARY KEY,
    queue_name TEXT,
    type TEXT,
    header_message_id TEXT,
    header_timestamp TEXT,
    header_correlation_id TEXT,
    metadata_retry_count INTEGER,
    metadata_ttl INTEGER,
    metadata_tags TEXT[],
    payload_event_type TEXT,
    payload_data TEXT,
    sender_user_name TEXT,
    sender_service TEXT,
    sender_name TEXT,
    message_status TEXT DEFAULT 'pending'
);