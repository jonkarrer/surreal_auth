use crate::application::SessionRepository;
use surrealdb::opt::RecordId;

pub async fn start_session<T: SessionRepository>(
    session_repository: &T,
    user_record_link: RecordId,
) -> Option<RecordId> {
    session_repository.create_session(user_record_link).await
}
