create table if not exists submission_file (
    id uuid primary key default gen_random_uuid(),
    name text,
    submission_id uuid,

    foreign key (submission_id) references submission(id) on delete cascade
);
