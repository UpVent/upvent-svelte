use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::ops::Deref;
