use rocket::serde::json::{json, Json, Value};
use crate::db::DbConnection;
use crate::db::models::brainlog::{BrainlogEntryType, NewBrainlogEntryType, BrainlogEntry, NewBrainlogEntry};

use diesel::prelude::*;

#[rocket::post("/brainlog/type/create", format = "json", data = "<data>")]
async fn type_create(conn: DbConnection, data: Json<NewBrainlogEntryType>) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let entry = conn.run(move |c|{
        diesel::insert_into(brainlog_entry_type::table)
        .values(&*data)
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error creating")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/type/get?<id>")]
async fn type_get(conn: DbConnection, id: &str) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn.run(move |c|{
        brainlog_entry_type::table
        .filter(brainlog_entry_type::id.eq(uuid))
        .first(c)
        .expect("Issue")
    }).await;

    Json(entry)
}

#[rocket::post("/brainlog/type/update?<id>", format = "json", data = "<data>")]
async fn type_update(conn: DbConnection, id: &str, data: Json<NewBrainlogEntryType>) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn.run(move |c|{
        diesel::update(brainlog_entry_type::table)
        .filter(brainlog_entry_type::id.eq(uuid))
        .set(&*data)
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error updating")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/type/delete?<id>")]
async fn type_delete(conn: DbConnection, id: &str) {
    use crate::db::schema::brainlog_entry_type;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    conn.run(move |c|{
        diesel::delete(brainlog_entry_type::table)
        .filter(brainlog_entry_type::id.eq(uuid))
        .execute(c)
        .expect("Error deleting")
    }).await;
}

#[rocket::get("/brainlog/type/list?<page>&<pagesize>")]
async fn type_list(conn: DbConnection, page: i64, pagesize: i64) -> Value {
    use diesel::dsl::count_star;
    use crate::db::schema::brainlog_entry_type::dsl::*;

    let rows: i64 = conn.run(|c|{
        brainlog_entry_type
        .select(count_star())
        .first(c)
        .expect("Issue")
    }).await;

    let items = conn.run(move |c|{
        brainlog_entry_type
        .limit(pagesize)
        .offset(page * pagesize)
        .select(BrainlogEntryType::as_select())
        .load(c)
        .expect("Issue")
    }).await;

    json!({
        "total_items": rows,
        "items": items,
    })
}

#[rocket::post("/brainlog/create", format = "json", data = "<data>")]
async fn log_create(conn: DbConnection, data: Json<NewBrainlogEntry>) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let entry = conn.run(move |c|{
        diesel::insert_into(brainlog_entry::table)
        .values(&*data)
        .returning(BrainlogEntry::as_returning())
        .get_result(c)
        .expect("Error creating")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/get?<id>")]
async fn log_get(conn: DbConnection, id: &str) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn.run(move |c|{
        brainlog_entry::table
        .filter(brainlog_entry::id.eq(uuid))
        .first(c)
        .expect("Issue")
    }).await;

    Json(entry)
}

#[rocket::post("/brainlog/update?<id>", format = "json", data = "<data>")]
async fn log_update(conn: DbConnection, id: &str, data: Json<NewBrainlogEntry>) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn.run(move |c|{
        diesel::update(brainlog_entry::table)
        .filter(brainlog_entry::id.eq(uuid))
        .set(&*data)
        .returning(BrainlogEntry::as_returning())
        .get_result(c)
        .expect("Error updating")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/delete?<id>")]
async fn log_delete(conn: DbConnection, id: &str) {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    conn.run(move |c|{
        diesel::delete(brainlog_entry::table)
        .filter(brainlog_entry::id.eq(uuid))
        .execute(c)
        .expect("Error deleting")
    }).await;
}

#[rocket::get("/brainlog/list?<page>&<pagesize>")]
async fn log_list(conn: DbConnection, page: i64, pagesize: i64) -> Value {
    use diesel::dsl::count_star;
    use crate::db::schema::brainlog_entry::dsl::*;

    let rows: i64 = conn.run(|c|{
        brainlog_entry
        .select(count_star())
        .first(c)
        .expect("Issue")
    }).await;

    let items = conn.run(move |c|{
        brainlog_entry
        .limit(pagesize)
        .offset(page * pagesize)
        .select(BrainlogEntry::as_select())
        .load(c)
        .expect("Issue")
    }).await;

    json!({
        "total_items": rows,
        "items": items,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        // BrainlogEntryType
        type_create,
        type_get,
        type_update,
        type_delete,
        type_list,
        // BrainlogEntry
        log_create,
        log_get,
        log_update,
        log_delete,
        log_list
    ]
}
