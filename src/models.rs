use std::fmt::Display;

use crate::schema::users::{self};

// We are going to define the struct as insertable to perform inserts on the table
#[derive(Insertable)]
#[table_name = "users"]
// For strings we have to make sure that they live as long as the struct
// So we use the 'a lifetime anotation. Or it will point to empty space on memory
pub struct NewUser<'a>{
    pub name: &'a str,
    pub email: &'a str,
    pub maried: Option<bool>,
    pub removed: bool,
    pub gender: &'a str,
}

impl<'a> NewUser<'a>{
    pub fn new(
        name: &'a str, email: &'a str, is_maried: Option<bool>, is_removed: bool, gender: &'a str
    ) -> NewUser<'a>{
        NewUser { 
            name: name, 
            email: email, 
            maried: is_maried, 
            removed: is_removed, 
            gender: gender
        }
    }
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User{
    pub uuid: i32,
    pub name: String,
    pub email: String,
    pub maried: Option<bool>,
    pub removed: bool,
    pub gender: String,
}
