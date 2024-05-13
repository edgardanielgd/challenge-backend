CREATE TABLE note_has_category (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL,
    cat_id UUID NOT NULL,

    CONSTRAINT fk_note_id
        FOREIGN KEY (note_id) REFERENCES note (id) ON DELETE CASCADE,

    CONSTRAINT fk_cat_id
        FOREIGN KEY (cat_id) REFERENCES category (id) ON DELETE CASCADE
)