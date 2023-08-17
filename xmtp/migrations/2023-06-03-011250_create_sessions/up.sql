CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    peer_installation_id TEXT NOT NULL,
    vmac_session_data BLOB NOT NULL
)
