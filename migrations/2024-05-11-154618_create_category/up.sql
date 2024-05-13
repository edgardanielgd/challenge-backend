CREATE TABLE category (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    owner_id UUID NOT NULL,

    CONSTRAINT fk_owner
        FOREIGN KEY (owner_id) REFERENCES app_user (id) ON DELETE CASCADE
)