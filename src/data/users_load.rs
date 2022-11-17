use diesel::{PgConnection};
use crate::{
    models::NewUser,
    sql_operations::user_ops::{add_user, get_user_by_email},
};

// Trying to add users with dataload
pub fn data_load_users(connection: &PgConnection){
    // We pass to is_maried field Some(false) cause it can be None, 
    // so we have to check if it have some boolean else it will be None
    // Here are some structs of NewUser our "InputObjectType" where we are preparating data
    // To be inserted in the right way on the database
    let jaw_user = NewUser::new(
        "Jaw", 
        "jaw@email.com.br", 
        Some(false), 
        false, 
        "Men i guess"
    );

    let hell_user = NewUser::new(
        "Hell",
        "hell@email.com.br",
        Some(false),
        false,
        "Men"
    );

    let motta_user = NewUser::new(
        "Motta",
        "motta@email.com.br",
        Some(false),
        false,
        "Men"
    );

    let murilo_user = NewUser::new(
        "Lumiro",
        "miluro@email.com.br",
        Some(false),
        false,
        "It depends"
    );

    let users_list: Vec<NewUser> = vec![
        jaw_user,
        motta_user,
        hell_user,
        murilo_user
    ];

    for user in users_list{
        // Here is our validation to see if user already exists on the DB,
        // So the function will return a Optin<User> or in the Pythonic way User | None
        // Knowing it we have to check if the return exists or not in the match statement 
        let db_user = get_user_by_email(connection, user.email);
        
        // The match is one of the most magical types in Rust were you can make a good number of validations
        // Based in on object of data
        // Here we are cheking if the db_user: Option<User> is getting some user,
        // Or the returned value is None, if its none it will add the user on DB
        // If it exists it will only log to us, that te record already exists
        match db_user {
            Some(user) => {print!("Skipping already loaded user {:?} \n", &user.email)},
            None => {
                add_user(&user, connection);
                print!("Adding new user to Database! User(name: {:?}, email: {:?}) \n", &user.name, &user.email)
            },
        }
    }
}
