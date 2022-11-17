use data::data_load_all;
use crate::sql_operations::user_ops::{get_users_total_count, get_all_db_users_with_filters};

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
    data_load_all(&connection);
    let db_users = get_all_db_users_with_filters(&connection);
    let total_users = get_users_total_count(&connection);

    print!("\nWe have in total {:?} users in owr database already\n", total_users);
    for (index, user )in db_users.iter().enumerate(){
        print!("{:?}-{:?} \n", index+1, user);
    }

}
