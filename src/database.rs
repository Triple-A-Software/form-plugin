use sqlx::{Executor, Postgres};

use crate::model::{Email, Form, Submission};

pub async fn list_forms(db: impl Executor<'_, Database = Postgres>) -> sqlx::Result<Vec<Form>> {
    sqlx::query_as(r#"select * from form"#).fetch_all(db).await
}

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
            captcha_callback,
            captcha_expired_callback,
            props,
            additional_html,
            email_attach_files
        ) values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
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
    .bind(form.captcha_callback)
    .bind(form.captcha_expired_callback)
    .bind(form.props)
    .bind(form.additional_html)
    .bind(form.email_attach_files)
    .fetch_one(db)
    .await
}

pub async fn update_form(
    id: i32,
    form: Form,
    db: impl Executor<'_, Database = Postgres>,
) -> sqlx::Result<Form> {
    sqlx::query_as(
        r#"update form set
            name = $1,
            send_confirmation_email = $2,
            confirmation_email_id = $3,
            confirmation_email_recipient_field_name = $4,
            send_submission_as_email_enabled = $5,
            send_submission_as_email = $6,
            submissions_email_id = $7,
            activate_cleverreach = $8,
            cleverreach_condition = $9,
            cleverreach_group_id = $10,
            cleverreach_email_field_name = $11,
            cleverreach_firstname_field_name = $12,
            cleverreach_lastname_field_name = $13,
            cleverreach_salutation = $14,
            activate_captcha = $15,
            captcha_callback = $16,
            captcha_expired_callback = $17,
            props = $18,
            additional_html = $19,
            email_attach_files = $20
        where id = $21
        returning *"#,
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
    .bind(form.captcha_callback)
    .bind(form.captcha_expired_callback)
    .bind(form.props)
    .bind(form.additional_html)
    .bind(form.email_attach_files)
    .bind(id)
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
) -> sqlx::Result<Submission> {
    sqlx::query_as(r#"insert into submission (id, form_id, data) values ($1, $2, $3) returning *"#)
        .bind(submission.id)
        .bind(submission.form_id)
        .bind(submission.data)
        .fetch_one(db)
        .await
}
