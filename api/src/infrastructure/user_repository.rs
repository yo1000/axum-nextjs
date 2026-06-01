use crate::domain::Mutation::{Assign, Clear, Retain};
use crate::domain::UserCommand::{Create, Modify, Replace};
use crate::domain::{Gender, Page, Pageable, User, UserCommand, UserRepository};
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::postgres::PgArguments;
use sqlx::{query, query_as, query_scalar, query_with, Arguments, AssertSqlSafe, FromRow, PgPool};
use uuid::Uuid;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    given_name: String,
    family_name: String,
    age: Option<i32>,
    gender: i32,
}

impl UserRow {
    fn to_user(&self) -> anyhow::Result<User> {
        Ok(User::of(
            self.id,
            self.username.clone(),
            self.given_name.clone(),
            self.family_name.clone(),
            self.age,
            Gender::try_from(self.gender)?,
        ))
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<User>> {
        TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, UserRow>(r#"
                    SELECT
                        id,
                        username,
                        given_name,
                        family_name,
                        age,
                        gender
                    FROM
                        users users
                    WHERE
                        id = $1
                    "#)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            })
            .await?
            .map(|row| row.to_user())
            .transpose()
    }

    async fn find_by_username(&self, username: &String) -> anyhow::Result<Option<User>> {
        TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, UserRow>(r#"
                    SELECT
                        id,
                        username,
                        given_name,
                        family_name,
                        age,
                        gender
                    FROM
                        users users
                    WHERE
                        username = $1
                    "#)
                    .bind(username)
                    .fetch_optional(conn)
                    .await
            })
            .await?
            .map(|row| row.to_user())
            .transpose()
    }

    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        users users
                    "#)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let users = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, UserRow>(r#"
                    SELECT
                        id,
                        username,
                        given_name,
                        family_name,
                        age,
                        gender
                    FROM
                        users users
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?
            .into_iter()
            .map(|row| row.to_user())
            .collect::<anyhow::Result<Vec<User>>>()?;

        Ok(Page::new(users, pageable, count))
    }

    async fn find_all_by_username_starting_with(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        users users
                    WHERE
                        username LIKE $1 || '%'
                    "#)
                    .bind(username)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let users = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, UserRow>(r#"
                    SELECT
                        id,
                        username,
                        given_name,
                        family_name,
                        age,
                        gender
                    FROM
                        users users
                    WHERE
                        username LIKE $1 || '%'
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(username)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?
            .into_iter()
            .map(|row| row.to_user())
            .collect::<anyhow::Result<Vec<User>>>()?;

        Ok(Page::new(users, pageable, count))
    }

    async fn save(&self, user_command: &UserCommand) -> anyhow::Result<User> {
        let id = match user_command {
            Create(user) => {
                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query(r#"
                            INSERT INTO users (
                                id,
                                username,
                                given_name,
                                family_name,
                                age,
                                gender
                            ) VALUES (
                                $1,
                                $2,
                                $3,
                                $4,
                                $5,
                                $6
                            )
                            "#)
                            .bind(user.id)
                            .bind(&user.username)
                            .bind(&user.given_name)
                            .bind(&user.family_name)
                            .bind(user.age)
                            .bind(user.gender.value())
                            .execute(conn)
                            .await
                    })
                    .await?;

                user.id
            }
            Replace(user) => {
                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query(r#"
                            UPDATE users
                            SET
                                username = $2,
                                given_name = $3,
                                family_name = $4,
                                age = $5,
                                gender = $6
                            WHERE
                                id = $1
                            "#)
                            .bind(user.id)
                            .bind(&user.username)
                            .bind(&user.given_name)
                            .bind(&user.family_name)
                            .bind(user.age)
                            .bind(user.gender.value())
                            .execute(conn)
                            .await
                    })
                    .await?;

                user.id
            }
            Modify(user) => {
                if user.is_all_retained() {
                    return self.find_by_id(&user.id)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Not found"))
                }

                let mut args = PgArguments::default();
                let mut arg_queries = Vec::new();
                let mut arg_index = 2; // Use $1 in WHERE clause
                let _ = args.add(user.id);

                match &user.username {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<String>);
                        arg_queries.push(format!("username = ${arg_index}"));
                        arg_index += 1;
                    }
                    Assign(param) => {
                        let _ = args.add(param);
                        arg_queries.push(format!("username = ${arg_index}"));
                        arg_index += 1;
                    }
                }

                match &user.given_name {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<String>);
                        arg_queries.push(format!("given_name = ${arg_index}"));
                        arg_index += 1;
                    }
                    Assign(param) => {
                        let _ = args.add(param);
                        arg_queries.push(format!("given_name = ${arg_index}"));
                        arg_index += 1;
                    }
                }

                match &user.family_name {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<String>);
                        arg_queries.push(format!("family_name = ${arg_index}"));
                        arg_index += 1;
                    }
                    Assign(param) => {
                        let _ = args.add(param);
                        arg_queries.push(format!("family_name = ${arg_index}"));
                        arg_index += 1;
                    }
                }

                match &user.age {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<i32>);
                        arg_queries.push(format!("age = ${arg_index}"));
                        arg_index += 1;
                    }
                    Assign(param) => {
                        let _ = args.add(param);
                        arg_queries.push(format!("age = ${arg_index}"));
                        arg_index += 1;
                    }
                }

                match &user.gender {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<i32>);
                        arg_queries.push(format!("gender = ${arg_index}"));
                        // arg_index += 1;  // Not use
                    }
                    Assign(param) => {
                        let _ = args.add(param.value());
                        arg_queries.push(format!("gender = ${arg_index}"));
                        // arg_index += 1;  // Not use
                    }
                }

                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query_with(
                            AssertSqlSafe(format!(r#"
                                UPDATE users
                                SET
                                    {}
                                WHERE
                                    id = $1
                                "#, arg_queries.join(", ")
                            )),
                            args)
                            .execute(conn)
                            .await
                    })
                    .await?;

                user.id
            }
        };

        self.find_by_id(&id).await?
            .ok_or_else(|| anyhow::anyhow!("Not found"))
    }
}
