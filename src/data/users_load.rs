use diesel::{PgConnection};
use crate::{
    models::NewUser,
    sql_operations::user_ops::{add_user, get_user_by_email},
};

// Trying to add users with dataload
pub fn data_load_users(connection: &PgConnection){
    // We pass to is_maried field Some(false) cause it can be None, 
    // so we have to check if it have some boolean else it will be None
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
        let db_user = get_user_by_email(connection, user.email);

        match db_user {
            Some(user) => {print!("Skipping already loaded user {:?} \n", &user.email)},
            None => {
                add_user(&user, connection);
                print!("Adding new user to Database! User(name: {:?}, email: {:?}) \n", &user.name, &user.email)
            },
        }
    }
}
