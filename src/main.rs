use data::data_load_all;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod schema;
mod models;
mod sql_operations;
mod data;

fn main() {
    let connection = db::establish_connection();
    data_load_all(connection);
}
