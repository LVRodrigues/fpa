use log::trace;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, Statement,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{configuration::Configuration, error::Error};

#[derive(Clone, Debug)]
pub struct AppState {
    configuration: Configuration,
    connection: DatabaseConnection,
}

impl AppState {
    pub fn new(configuration: Configuration, connection: DatabaseConnection) -> Self {
        Self {
            configuration,
            connection,
        }
    }

    pub async fn connection(&self, tenant: &Uuid) -> Result<DatabaseTransaction, Error> {
        trace!("New database connection.");
        let db = &self.connection;
        db.ping().await.map_err(|error| {
            log::error!("Database ping failed: {error}");
            Error::DatabaseConnection
        })?;

        let trx = db.begin().await.map_err(|error| {
            log::error!("Failed to begin database transaction: {error}");
            Error::DatabaseConnection
        })?;

        // A transaction-local setting prevents tenant context leaking through a
        // pooled connection after commit/rollback. Bind the value instead of
        // interpolating it into SQL.
        let tenant = tenant.to_string();
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT set_config('app.current_tenant', $1, true)",
            [tenant.into()],
        );
        trx.execute_raw(statement).await.map_err(|error| {
            log::error!("Failed to set database tenant context: {error}");
            Error::DatabaseConnection
        })?;

        Ok(trx)
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
