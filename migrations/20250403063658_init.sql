create table if not exists email (
    id int generated always as identity,
    subject text not null,
    content text not null,

    primary key (id)
);

create table if not exists form (
    id int generated always as identity,
    name text not null,
    send_confirmation_email boolean not null default false,
    confirmation_email_id int,
    confirmation_email_recipient_field_name text,
    send_submission_as_email_enabled boolean not null default false,
    send_submission_as_email text,
    submissions_email_id int,
    activate_cleverreach boolean not null default false,
    cleverreach_condition jsonb not null default '{"type":"OnSubmit"}'::jsonb,
    cleverreach_group_id text,
    cleverreach_email_field_name text,
    cleverreach_firstname_field_name text,
    cleverreach_lastname_field_name text,
    cleverreach_salutation text,
    activate_captcha boolean not null default false,
    captcha_type text,
    captcha_callback text,
    captcha_expired_callback text,
    props jsonb not null default '[]'::jsonb,

    primary key (id),
    foreign key (confirmation_email_id) references email(id)
);

create table if not exists submission (
    id int generated always as identity,
    form_id int not null,
    "timestamp" timestamp not null default NOW(),
    data jsonb not null default '{}'::jsonb,

    foreign key (form_id) references form(id)
);

create table if not exists settings (
    id text not null primary key,
    cleverreach_client_id text,
    cleverreach_secret text,
    h_captcha_secret text,
    re_captcha_secret text,
    captcha_sitekey text
);
