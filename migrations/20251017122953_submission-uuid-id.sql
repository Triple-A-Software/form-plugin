alter table submission add column new_id uuid;
update submission set new_id = gen_random_uuid();
alter table submission drop column id;
alter table submission rename column new_id to id;
alter table submission add primary key (id);
alter table submission alter column id set default gen_random_uuid();
