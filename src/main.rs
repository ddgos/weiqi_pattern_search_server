#[macro_use]
extern crate rocket;

use std::fmt::Display;

use rocket::fairing::{self, AdHoc};
use rocket::request::FromParam;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};

use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection, Database};

use futures::{future::TryFutureExt, stream::TryStreamExt};

use weiqi_pattern::{Pattern, PatternParseError};

#[derive(Database)]
#[database("patterns")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

struct PatternWrapper(Pattern);

impl PatternWrapper {
    fn get_inner(&self) -> &Pattern {
        &self.0
    }

    fn to_inner(self) -> Pattern {
        self.0
    }
}

impl FromParam<'_> for PatternWrapper {
    type Error = PatternParseError;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        // todo: add logging
        let pattern: Pattern = param.parse()?;
        Ok(Self(pattern))
    }
}

impl Display for PatternWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_inner().repr())
    }
}

#[get("/")]
fn index() -> &'static str {
    "
        Welcome to the weiqi pattern search server!
        Hopefully we can direct you to some useful go learning content.
    
        see /api for api access to the pattern search
    "
}

#[get("/api")]
fn api_index() -> &'static str {
    "
        apis available:
         - v1 
    "
}

#[get("/api/v1")]
fn api_v1_index() -> &'static str {
    "
        endpoints available for this api:
         - search/<pattern>
           where <pattern> is a pattern as defined by
           https://github.com/ddgos/weiqi_pattern
    "
}

#[get("/api/v1/search/<pattern>")]
async fn api_v1_search(mut db: Connection<Db>, pattern: PatternWrapper) -> Option<String> {
    format!(
        "
            you requested a search with pattern {}
            returning results is not yet supported
        ",
        pattern
    );
    let lec_ids = sqlx::query("SELECT (lec_id, pat) FROM patterns")
        .fetch(&mut **db)
        .map_ok(|record| {
            let lec_id: i64 = record.try_get(0).unwrap();
            let pat_str: String = record.try_get(1).unwrap();
            let pat: Pattern = pat_str.parse().unwrap();

            (lec_id, pat)
        })
        .try_collect::<Vec<_>>()
        .await;

    Some(format!("{:?}", lec_ids))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, api_index, api_v1_index, api_v1_search])
}
