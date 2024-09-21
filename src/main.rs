#[macro_use]
extern crate rocket;

use std::{collections::HashMap, fmt::Display};

use rocket::request::FromParam;
use rocket::serde::{json::Json, Serialize};

use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection, Database};

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

    fn into_inner(self) -> Pattern {
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
           https://github.com/ddgos/weiqi_pattern.
           this will return a JSON formatted list of objects with fields
             lecture_id int
               the IGS lecture ID
             match_cost int
               the minimum match cost of patterns in lecture <lecture_id> and
               <pattern>
    "
}

#[get("/api/v1/search/<pattern>")]
async fn api_v1_search(
    mut db: Connection<Db>,
    pattern: PatternWrapper,
) -> Option<Json<Vec<CostedResource>>> {
    let db_result = sqlx::query("SELECT lec_id, pat FROM patterns")
        .fetch_all(&mut **db)
        .await;
    let db_rows = match db_result {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("failed to fetch db results as {}", e);
            return None;
        }
    };

    let patterns = db_rows.into_iter().filter_map(|record| {
        let lec_id_get_result: Result<i64, _> = record.try_get(0);
        let lec_id = match lec_id_get_result {
            Ok(id) => id,
            Err(e) => {
                eprintln!("failed to get lec_id as {}", e);
                return None;
            }
        };
        let pat_str_get_result: Result<String, _> = record.try_get(1);
        let pat_str = match pat_str_get_result {
            Ok(str) => str,
            Err(e) => {
                eprintln!("failed to get pat as {}", e);
                return None;
            }
        };
        let pat: Pattern = pat_str.parse().ok()?;

        Some((lec_id, pat))
    });

    let costed_lecs = sort_patterns(pattern.into_inner(), patterns);

    Some(Json(costed_lecs))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct CostedResource {
    lecture_id: i64,
    match_cost: u64,
}

fn sort_patterns(
    needle: Pattern,
    patterns: impl Iterator<Item = (i64, Pattern)>,
) -> Vec<CostedResource> {
    let mut resource_best_costs = HashMap::<i64, u64>::new();
    let id_costs = patterns.filter_map(|(id, pat)| {
        needle
            .minimum_positioned_variation_match_cost(&pat)
            .map(|(_, _, cost)| (id, cost))
    });
    for (resource_id, cost) in id_costs {
        let maybe_to_insert = match resource_best_costs.get(&resource_id) {
            Some(best_yet_cost) => {
                if cost < *best_yet_cost {
                    Some(cost)
                } else {
                    None
                }
            }
            None => Some(cost),
        };

        if let Some(to_insert) = maybe_to_insert {
            resource_best_costs.insert(resource_id, to_insert);
        }
    }

    let mut resource_best_costs: Vec<_> = resource_best_costs
        .into_iter()
        .map(|(lecture_id, match_cost)| CostedResource {
            lecture_id,
            match_cost,
        })
        .collect();
    resource_best_costs.sort_unstable_by_key(|CostedResource { match_cost, .. }| *match_cost);

    resource_best_costs
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, api_index, api_v1_index, api_v1_search])
}
