use sqlx::{Executor, Postgres, postgres::PgQueryResult};

use crate::model::{Email, Form, Submission};

pub async fn create_form(
    form: Form,
    db: impl Executor<'_, Database = Postgres>,
) -> sqlx::Result<Form> {
    sqlx::query_as(
        r#"insert into form (
            name,
            send_confirmation_email,
            confirmation_email_id,
            confirmation_email_recipient_field_name,
            send_submission_as_email_enabled,
            send_submission_as_email,
            submissions_email_id,
            activate_cleverreach,
            cleverreach_condition,
            cleverreach_group_id,
            cleverreach_email_field_name,
            cleverreach_firstname_field_name,
            cleverreach_lastname_field_name,
            cleverreach_salutation,
            activate_captcha,
            captcha_type,
            captcha_callback,
            captcha_expired_callback,
            props
        ) values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19
        ) returning *"#,
    )
    .bind(form.name)
    .bind(form.send_confirmation_email)
    .bind(form.confirmation_email_id)
    .bind(form.confirmation_email_recipient_field_name)
    .bind(form.send_submission_as_email_enabled)
    .bind(form.send_submission_as_email)
    .bind(form.submissions_email_id)
    .bind(form.activate_cleverreach)
    .bind(form.cleverreach_condition)
    .bind(form.cleverreach_group_id)
    .bind(form.cleverreach_email_field_name)
    .bind(form.cleverreach_firstname_field_name)
    .bind(form.cleverreach_lastname_field_name)
    .bind(form.cleverreach_salutation)
    .bind(form.activate_captcha)
    .bind(form.captcha_type)
    .bind(form.captcha_callback)
    .bind(form.captcha_expired_callback)
    .bind(form.props)
    .fetch_one(db)
    .await
}

pub async fn create_email(
    email: Email,
    db: impl Executor<'_, Database = Postgres>,
) -> sqlx::Result<Email> {
    sqlx::query_as(
        r#"insert into email (
            subject,
            content
        ) values ($1, $2) returning *"#,
    )
    .bind(email.subject)
    .bind(email.content)
    .fetch_one(db)
    .await
}

pub async fn create_submission(
    submission: Submission,
    db: impl Executor<'_, Database = Postgres>,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(r#"insert into submission (form_id, data) values ($1, $2)"#)
        .bind(submission.form_id)
        .bind(submission.data)
        .execute(db)
        .await
}
