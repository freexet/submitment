use color_eyre::Result;
use nanoid::nanoid;

use crate::repository::Repository;
use crate::schema::submission::{Submission, SubmissionForm};

#[derive(Clone)]
pub struct SubmissionService {
    repository: Repository,
}

impl SubmissionService {
    pub fn new(repository: Repository) -> Self {
        SubmissionService { repository }
    }

    pub async fn get_submissions(&self) -> Result<Vec<Submission>> {
        Ok(self.repository.get_all_submissions().await?)
    }

    pub async fn submit_submission(&self, user_id: &str, question: &str, answer: &str) -> Result<Submission> {
        let form = SubmissionForm {
            id: nanoid!(),
            user_id,
            question,
            answer
        };

        Ok(self.repository.insert_new_submission(form).await?)
    }

    pub async fn get_submission_by_id(&self, id: &str) -> Result<Submission> {
        Ok(self.repository.get_submission_by_id(id).await?)
    }
}
