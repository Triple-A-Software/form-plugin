-- large uploads would exceed mail server limits, so attaching them to the submission email can be turned off
alter table form add column email_attach_files boolean not null default true;
