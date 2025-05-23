use summeriq_backend::{
    config::Config,
    db,
    services::auth::AuthService,
    models::user::{CreateUser, LoginUser},
};
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    let config = Config::from_env().expect("Failed to load configuration");
    db::create_pool(&config.database_url)
        .await
        .expect("Failed to connect to database")
}

#[tokio::test]
async fn test_user_registration() {
    let pool = setup_test_db().await;
    let auth_service = AuthService::new(pool.clone(), "test_secret".to_string());

    let user = CreateUser {
        name: "Test User".to_string(),
        email: format!("test_{}@example.com", Uuid::new_v4()),
        password: "password123".to_string(),
    };

    let result = auth_service.register(user).await;
    assert!(result.is_ok());

    let created_user = result.unwrap();
    assert_eq!(created_user.name, "Test User");
}

#[tokio::test]
async fn test_user_login() {
    let pool = setup_test_db().await;
    let auth_service = AuthService::new(pool.clone(), "test_secret".to_string());

    // First register a user
    let user = CreateUser {
        name: "Test User".to_string(),
        email: format!("test_{}@example.com", Uuid::new_v4()),
        password: "password123".to_string(),
    };

    let _ = auth_service.register(user.clone()).await.unwrap();

    // Then try to login
    let login = LoginUser {
        email: user.email,
        password: user.password,
    };

    let result = auth_service.login(login).await;
    assert!(result.is_ok());

    let (user, token) = result.unwrap();
    assert_eq!(user.name, "Test User");
    assert!(!token.is_empty());
}
