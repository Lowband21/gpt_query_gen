use serde::Serialize;
use diesel::Queryable;


#[derive(Debug, Queryable, Serialize)]
pub struct Project {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Skill {
    id: i32,
    name: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Bio {
    id: i32,
    bio_content: String,
}
