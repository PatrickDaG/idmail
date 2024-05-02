use leptos::*;
use leptos_icons::Icon;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    /// The username / mailbox address
    pub username: String,
    /// The associated password hash
    pub password: String,
    /// Whether the user is a mailbox
    pub mailbox: bool,
    /// Whether the user is an admin
    pub admin: bool,
    /// Whether the user is active
    pub active: bool,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use super::User;
    use anyhow::{anyhow, Context};
    pub use axum_session_auth::{Authentication, HasPermission, SessionSqlitePool};
    pub use sqlx::SqlitePool;
    pub use std::collections::HashSet;
    pub type AuthSession = axum_session_auth::AuthSession<User, String, SessionSqlitePool, SqlitePool>;
    pub use async_trait::async_trait;
    pub use bcrypt::{hash, verify, DEFAULT_COST};

    impl User {
        pub async fn get(username: &str, pool: &SqlitePool) -> Option<Self> {
            let user = sqlx::query_as::<_, User>(
                "SELECT username, password, FALSE AS mailbox, admin, active \
                FROM users WHERE username = $1 \
                UNION SELECT address AS username, password, TRUE AS mailbox, FALSE AS admin, active \
                FROM mailboxes WHERE address = $1",
            )
            .bind(username)
            .fetch_one(pool)
            .await
            .ok()?;

            Some(user)
        }
    }

    #[async_trait]
    impl Authentication<User, String, SqlitePool> for User {
        async fn load_user(username: String, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.context("Missing sql pool")?;

            User::get(&username, pool)
                .await
                .ok_or_else(|| anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            self.active
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, _perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            false
        }
    }
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = crate::database::ssr::auth()?;
    Ok(auth.current_user)
}

/// Get the current user and ensure that it is an admin
#[server]
pub async fn auth_admin() -> Result<User, ServerFnError> {
    let user = get_user().await?.ok_or_else(|| ServerFnError::new("Unauthorized"))?;
    if !user.admin {
        return Err(ServerFnError::new("Unauthorized"));
    }

    Ok(user)
}

/// Ensure that the user is logged in
#[server]
pub async fn auth_any_user() -> Result<User, ServerFnError> {
    let user = get_user().await?.ok_or_else(|| ServerFnError::new("Unauthorized"))?;
    Ok(user)
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let pool = crate::database::ssr::pool()?;
    let auth = crate::database::ssr::auth()?;

    let user = User::get(&username, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("Wrong password or invalid user"))?;

    auth.login_user(user.username);
    auth.remember_user(false);
    leptos_axum::redirect("/");
    Ok(())
    // TODO match bcrypt::verify(password, &user.password)? {
    // TODO     true => {
    // TODO         auth.login_user(user.username);
    // TODO         auth.remember_user(false);
    // TODO         leptos_axum::redirect("/");
    // TODO         Ok(())
    // TODO     }
    // TODO     false => Err(ServerFnError::ServerError("Password does not match.".to_string())),
    // TODO }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = crate::database::ssr::auth()?;
    auth.logout_user();
    leptos_axum::redirect("/");
    Ok(())
}

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    let action_value = Signal::derive(move || action.value().get().unwrap_or(Ok(())));

    view! {
        <div class="relative flex min-h-screen flex-col bg-background">
            <div class="w-full h-screen flex items-center justify-center px-4">
                <div class="flex flex-col mx-auto">
                    <div class="mx-auto mb-4 flex flex-row">
                        <h2 class="text-4xl leading-none font-bold bg-gradient-to-br from-purple-600 to-blue-500 inline-block text-transparent bg-clip-text">idmail</h2>
                        <Icon icon=icondata::IoMail class="ml-1 w-6 h-6"/>
                    </div>
                    <ActionForm action class="rounded-lg border border-[1.5px] text-card-foreground max-w-sm">
                        <div class="flex flex-col space-y-1.5 p-6">
                            <h2 class="font-semibold tracking-tight text-2xl mb-2">Login</h2>
                            <p class="text-sm text-gray-500">"Enter your mailbox address and password below to login"</p>
                        </div>
                        <div class="p-6 pt-0">
                            <div class="grid gap-4">
                                <div class="grid gap-2">
                                    <label
                                        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                        for="username"
                                    >
                                        Email
                                    </label>
                                    <input
                                        class="flex flex-none w-full rounded-lg border-[1.5px] border-input bg-transparent text-sm p-2.5 transition-all focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                                        type="text"
                                        name="username"
                                        placeholder="username@example.com"
                                        required="required"
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <div class="flex items-center">
                                        <label
                                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                            for="password"
                                        >
                                            Password
                                        </label>
                                    </div>
                                    <input
                                        class="flex flex-none w-full rounded-lg border-[1.5px] border-input bg-transparent text-sm p-2.5 transition-all focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                                        type="password"
                                        name="password"
                                        required="required"
                                    />
                                </div>
                                <ErrorBoundary fallback=|errors| {
                                    view! {
                                        <div class="rounded-lg p-4 flex bg-red-100">
                                            <div>
                                                <Icon icon=icondata::BiXCircleSolid class="w-5 h-5 text-red-400"/>
                                            </div>
                                            <div class="ml-3 text-red-700">
                                                <p>
                                                    {move || {
                                                        errors
                                                            .get()
                                                            .into_iter()
                                                            .map(|(_, e)| view! { {e.to_string()} })
                                                            .collect_view()
                                                    }}

                                                </p>
                                            </div>
                                        </div>
                                    }
                                }>

                                    {action_value}
                                </ErrorBoundary>
                                <button
                                    type="submit"
                                    tabindex="0"
                                    class="inline-flex w-full justify-center mt-3 items-center rounded-lg transition-all p-2.5 bg-blue-600 hover:bg-blue-500 font-semibold text-white focus:ring-4 focus:ring-blue-300 sm:w-auto"
                                >
                                    Login
                                </button>
                            </div>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
