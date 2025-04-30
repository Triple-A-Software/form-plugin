use fake::{Fake, Faker};
use form_plugin::{
    create_db,
    database::{create_email, create_form, create_submission},
    model::{Email, Form, Submission},
};

#[tokio::main]
async fn main() {
    let db = create_db().await;
    let fake_email: Email = Faker.fake();
    create_email(fake_email, &db).await.unwrap();
    let fake_form: Form = Faker.fake();
    create_form(fake_form, &db).await.unwrap();

    let fake_submissions = fake::vec![Submission; 500];
    for submission in fake_submissions {
        create_submission(submission, &db).await.unwrap();
    }
}
