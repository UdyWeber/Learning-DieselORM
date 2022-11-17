// In the use method we are importing things from external modules or packages in our Cargo.toml file
use data::data_load_all;
use crate::sql_operations::user_ops::{get_users_total_count, get_all_db_users_with_filters};

#[macro_use]
extern crate diesel;
extern crate dotenv;

// Talking abbout Rust stuff we have to mod our modules
// To be able to use it stuff and access them
// As you can see every folder has its own mod.rs it means that
// It is now a rust module, and inside of the mod.rs we import all our files
// Similar to a __init__.py file.
mod db;
mod schema;
mod models;
mod sql_operations;
mod data;

// Function main, were all the magic happens
fn main() {
    // This function is used similar to the Sqlalchemy session, it will allow us to communicate with DB
    let connection = db::establish_connection();

    // Here just loading some test DATA,
    // You can see that it takes as argument a reference to our DB connection
    data_load_all(&connection);

    // Here we are taking a vector of all users existent in our database
    let db_users = get_all_db_users_with_filters(&connection);

    // And here the user total_count to display it later
    let total_users = get_users_total_count(&connection);

    // Here we are just looping trought the records to display them on the terminal
    // Just fooling around and making some visual stuff to stimulate 
    print!("\nWe have in total {:?} users in owr database already\n", total_users);
    for (index, user )in db_users.iter().enumerate(){
        print!("{:?}-{:?} \n", index+1, user);
    }

}
