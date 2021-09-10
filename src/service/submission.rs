use color_eyre::Result;

use crate::repository::Repository;
use crate::schema::submission::Submission;

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
}
