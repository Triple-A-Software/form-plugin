-- host:port of a clamd instance; when set, every uploaded file is scanned and the submission is rejected unless clamd reports it clean
alter table settings add column clamav_address text;
