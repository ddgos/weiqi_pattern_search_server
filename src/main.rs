#[macro_use]
extern crate rocket;

use std::fmt::Display;

use rocket::request::FromParam;
use weiqi_pattern::{Pattern, PatternParseError};

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
fn api_v1_search(pattern: PatternWrapper) -> String {
    format!(
        "
            you requested a search with pattern {}
            returning results is not yet supported
        ",
        pattern
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, api_index, api_v1_index, api_v1_search])
}
