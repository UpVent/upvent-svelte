use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::ops::Deref;

// Create a wrapper for the r2d2 Pool object
pub type Pool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    diesel::r2d2::Pool::new(manager).expect("Failed to init database pool!")
}

// Create a guard for our database connection
pub struct Conn(pub diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Conn, ()> {
        let pool = try_outcome!(request.guard::<&State<Pool>>().await);
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = SqliteConnection;

    // Rust already inlines this, no need to specify the previous decorator.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
