use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use crate::models::*;

type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
