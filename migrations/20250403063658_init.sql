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
    activate_cleverreach boolean not null default false,
    cleverreach_group_id int,
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
    cleverreach_secret text
);
