alter table settings add column captcha_provider text;

-- backfill the global provider from whatever provider existing forms used most,
-- so installations that already had a captcha configured keep working
update settings
set captcha_provider = (
    select captcha_type
    from form
    where captcha_type is not null
    group by captcha_type
    order by count(*) desc
    limit 1
)
where captcha_provider is null;
