use fake::{Fake, Faker};
use form_plugin::{
    create_db,
    database::{create_email, create_form, create_submission, list_forms},
    model::{Email, Form, Submission},
};
use rand::{Rng, seq::IndexedRandom};

#[tokio::main]
async fn main() {
    let db = create_db().await;
    let fake_email: Email = Faker.fake();
    let fake_email_id = fake_email.id;
    create_email(fake_email, &db).await.unwrap();
    let mut fake_form: Form = Faker.fake();
    if fake_form.send_confirmation_email {
        fake_form.confirmation_email_id = Some(fake_email_id);
    }
    create_form(fake_form, &db).await.unwrap();

    let amount: usize = rand::rng().random_range(1..100);

    let forms: Vec<Form> = list_forms(&db).await.unwrap();

    let fake_submissions = fake::vec![Submission; amount];
    for mut submission in fake_submissions {
        let form_id = forms.choose(&mut rand::rng()).unwrap().id;
        submission.form_id = form_id;
        let _ = create_submission(submission, &db).await;
    }
}
