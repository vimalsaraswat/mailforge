-- Add migration script here
CREATE TABLE projects (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name TEXT NOT NULL,

    template_id UUID REFERENCES email_templates(id) ON DELETE SET NULL,
    mail_account_id UUID NOT NULL REFERENCES mail_accounts(id),

    objective TEXT,
    instructions TEXT,

    status TEXT NOT NULL DEFAULT 'draft',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_projects_user_id
    ON projects(user_id);

CREATE INDEX idx_projects_template_id
    ON projects(template_id);

CREATE INDEX idx_projects_mail_account_id
    ON projects(mail_account_id);
