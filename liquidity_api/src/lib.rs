pub use liquidity_elections as elections;
pub use liquidity_elections::ElectionResolvers;

use liquidity::{context::User, Connection, Context};
use std::{fmt, sync::Arc};

pub struct APIContext {
    db: Arc<Connection>,
    user: Option<User>,
    elections: Arc<ElectionResolvers>
}

impl APIContext {
    pub fn new(db: Arc<Connection>, user: Option<User>, elections: Arc<ElectionResolvers>) -> Self {
        APIContext {
            db,
            user,
            elections
        }
    }

    pub fn clone_with_user(&self, user: User) -> Self {
        APIContext {
            db: self.db.clone(),
            user: Some(user),
            elections: self.elections.clone()
        }
    }

    pub fn elections(&self) -> Arc<ElectionResolvers> {
        self.elections.clone()
    }
}

impl Clone for APIContext {
    fn clone(&self) -> Self {
        APIContext {
            db: self.db.clone(),
            user: self.user.clone(),
            elections: self.elections.clone()
        }
    }
}

impl Context<Arc<Connection>> for APIContext {
    fn db(&self) -> Arc<Connection> {
        self.db.clone()
    }
    fn user(&self) -> &Option<User> {
        &self.user
    }
}

impl fmt::Debug for APIContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user: {:?}", self.user)
    }
}
