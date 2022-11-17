use crate::schema::users::{self};

// We are going to define the struct as insertable to perform inserts on the table
#[derive(Insertable)]
#[table_name = "users"]
// For strings we have to make sure that they live as long as the struct
// So we use the 'a lifetime anotation. Or it will point to empty space on memory
// This Structure will be used as the InputObjectType from graphene, 
// That takes the arguments to be inserted in DB, 
// For that to be possible we need to derive the Trait Insertable from Diesel
pub struct NewUser<'a>{
    pub name: &'a str,
    pub email: &'a str,
    pub maried: Option<bool>,
    pub removed: bool,
    pub gender: &'a str,
}

// Here we are just creating an implementation to be able to create a new InputObject easely
// Notice that the impl takes the <'a> lifetime too, to it to be able to live as long as the struct that it implements into
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

// This model is similar to a GRAPHQL object type
// We are only going to use it on our queries,
// It will behave like our object to be Queried,
// It derives the Queryable trait that permits it to be used as Query object
#[derive(Queryable, Debug, AsChangeset)]
pub struct User{
    pub uuid: i32,
    pub name: String,
    pub email: String,
    pub maried: Option<bool>,
    pub removed: bool,
    pub gender: String,
}
