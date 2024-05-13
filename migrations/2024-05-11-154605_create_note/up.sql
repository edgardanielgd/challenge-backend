CREATE TYPE note_content_type AS ENUM(
    'markdown', 'plaintext'
);

CREATE TABLE note (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type note_content_type NOT NULL,
    archived BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    color TEXT NOT NULL,
    owner_id UUID NOT NULL,

    CONSTRAINT fk_owner_id
        FOREIGN KEY (owner_id) REFERENCES app_user(id) ON DELETE CASCADE
);