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
    props jsonb not null default '[]'::jsonb,

    primary key (id),
    foreign key (confirmation_email_id) references email(id)
);

