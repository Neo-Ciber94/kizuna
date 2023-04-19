use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[cfg_attr(feature = "postgres", derive(sqlx::FromRow))]
#[derive(Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn save(&mut self, input: CreateUser) -> anyhow::Result<User>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
}

#[cfg(feature = "memory")]
pub mod memory {
    use std::{
        collections::HashMap,
        ops::Deref,
        sync::{Arc, RwLock},
    };

    use crate::{CreateUser, User, UserRepository};

    #[derive(Clone, Default)]
    pub struct MemoryDb(Arc<RwLock<HashMap<i64, User>>>);
    impl Deref for MemoryDb {
        type Target = Arc<RwLock<HashMap<i64, User>>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub struct InMemoryUserRepository(MemoryDb);
    impl InMemoryUserRepository {
        pub fn new(db: MemoryDb) -> Self {
            InMemoryUserRepository(db)
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn save(&mut self, input: CreateUser) -> anyhow::Result<User> {
            let id = i64::try_from(self.0.read().unwrap().len()).unwrap() + 1;
            let user = User {
                id,
                username: input.username,
            };

            self.0.write().unwrap().insert(id, user.clone());
            Ok(user)
        }

        async fn get_all(&self) -> anyhow::Result<Vec<User>> {
            let users = self
                .0
                .read()
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<User>>();
            Ok(users)
        }
    }
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use crate::{CreateUser, User, UserRepository};
    use sqlx::{Pool, Postgres};
    pub struct PostgresUserRepository(Pool<Postgres>);
    impl PostgresUserRepository {
        pub fn new(pool: Pool<Postgres>) -> Self {
            PostgresUserRepository(pool)
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for PostgresUserRepository {
        async fn save(&mut self, input: CreateUser) -> anyhow::Result<User> {
            let pool = &self.0;
            let user = sqlx::query_as!(
                User,
                "INSERT INTO users (username) VALUES ($1) RETURNING *",
                &input.username
            )
            .fetch_one(pool)
            .await?;
            Ok(user)
        }

        async fn get_all(&self) -> anyhow::Result<Vec<User>> {
            let pool = &self.0;
            let users = sqlx::query_as!(User, "SELECT * FROM users")
                .fetch_all(pool)
                .await?;
            Ok(users)
        }
    }
}
