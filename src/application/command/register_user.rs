use crate::application::{UserRecordId, UserRepository};
use crate::domain::User;

pub async fn register_user<T: UserRepository>(
    repository: &T,
    new_user: &User,
) -> Option<UserRecordId> {
    match repository.find_user_by_email(&new_user.get_email()).await {
        Ok(user_record) => {
            println!("User {} Already Exists", user_record.user.get_email());
            return None;
        }
        Err(_) => {
            match repository
                .create_user(
                    &new_user.get_username(),
                    &new_user.get_password(),
                    &new_user.get_email(),
                )
                .await
            {
                Ok(record_id) => {
                    println!("User Has Been Created");
                    return Some(record_id);
                }
                Err(_) => {
                    println!("Failed to create new user");
                    return None;
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::DataStore;

    use super::*;

    #[tokio::test]
    async fn test_register_use_case() {
        // Start database
        let user_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-register@email.com";
        let new_user = User::new(username, password, email);
        let registration = register_user(&user_repo, &new_user).await;
        assert!(registration.is_some());

        // Test registration failure
        let new_user = User::new(username, password, email);
        let registration = register_user(&user_repo, &new_user).await;
        assert!(registration.is_none());
    }
}
