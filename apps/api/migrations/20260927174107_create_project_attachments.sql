-- Add migration script here
CREATE TABLE project_attachments (
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,

    PRIMARY KEY (project_id, attachment_id)
);
