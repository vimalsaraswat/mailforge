-- Add migration script here
CREATE TABLE project_recipients (
    id UUID PRIMARY KEY,

    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    email TEXT NOT NULL,
    data JSONB NOT NULL DEFAULT '{}'::jsonb,

    status TEXT NOT NULL DEFAULT 'pending',

    rendered_subject TEXT,
    rendered_body TEXT,

    sent_at TIMESTAMPTZ,
    error TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(project_id, email)
);

CREATE INDEX idx_project_recipients_project_id
    ON project_recipients(project_id);

CREATE INDEX idx_project_recipients_email
    ON project_recipients(email);

CREATE INDEX idx_project_recipients_status
    ON project_recipients(status);
