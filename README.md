# Surreal Auth

An authentication solution for SurrealDB.

## Roadmap

- [ ] Database
  - [ ] Authentication and Authorization
    - [ ] User Schema
    - [ ] Roles Schema
    - [ ] User Registration
    - [ ] User Authentication
    - [ ] Session Management
    - [ ] Authorization
    - [ ] Login/Logout
  - [ ] Recipes Schema

## Setup

Start Development Database, Non-Persist, No-Auth

```bash
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start
```

## Notes

### Step 1: Designing the Database Schema

Define a database schema to store user information. At a minimum, this schema should include fields for a user ID, username, and hashed password.

1. User table: This should include columns such as `id`, `username`, `hashed_password`, `email`, and `created_at`.
2. Role table (optional): This would store different roles that users can have within the system.
3. User_Role table (optional): A junction table to link users with their respective roles.

### Step 2: Implementing User Registration

Implement the user registration feature, which allows new users to create accounts.

1. Validation: Validate user inputs, like checking if the email format is correct.
2. Password hashing: Use a strong cryptographic hash function (like bcrypt or Argon2) to hash the user's password before storing it in the database.
3. User creation: Create a new user record in the database.

### Step 3: Implementing User Authentication

Implement the user authentication feature, which allows users to log in to their accounts.

1. Input collection: Collect the username and password from the user.
2. User lookup: Retrieve the user record from the database based on the username.
3. Password verification: Use the hash function to verify that the hashed version of the inputted password matches the hashed password stored in the database.
4. Session creation: If the password is correct, create a new session for the user.

### Step 4: Implementing Session Management

Implement session management to maintain a user's authenticated state across multiple requests.

1. Session creation: When a user successfully logs in, create a session and associate it with the user.
2. Session storage: Store the session information, either in a database or in-memory data store.
3. Session validation: Validate the session on each request to ensure that the user is authenticated.

### Step 5: Implementing Authorization (optional)

Implement authorization to control access to certain resources based on a user's roles or permissions.

1. Role assignment: Assign roles to users during registration or through an admin interface.
2. Permission checking: Check the user's permissions before allowing access to certain resources.

### Step 6: Implementing Logout Functionality

Implement a logout feature to allow users to end their sessions.

1. Session destruction: Destroy the user's session when they log out.
2. Cookie clearing: Clear any authentication cookies from the user's browser.

### Step 7: Testing

Test your authentication system thoroughly to identify and fix any vulnerabilities.

1. Unit testing: Write unit tests for individual components.
2. Integration testing: Write integration tests to test the system as a whole.

### Step 8: Deployment

Deploy your authentication service to a production environment, ensuring to use secure connections (HTTPS) and following other security best practices.

### Step 9: Maintenance

Regularly update your authentication system to patch vulnerabilities and improve security.

Remember, building a secure authentication system from scratch can be challenging, and it's often a good idea to use established libraries or frameworks to help with this. Also, always ensure to follow the latest security best practices and guidelines.
