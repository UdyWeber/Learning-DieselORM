use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::models::NewUser;
use crate::models::User;
use crate::schema::users::dsl::*;

// This function adds a User to the database, 
// It takes as argument a reference to a NewUser "ObjectType",
// And it takes the DBconnection to make the commit action
pub fn add_user(user: &NewUser, connection: &PgConnection){
    diesel::insert_into(users)
    .values(user)
    .execute(connection)
    .expect("Deu merda mané, não inseriu o usuário"); // Here we expect an Error in case it panic we see why and where
}

// Here is our function to get the total_count of the users in the Database
// You can see that the we have a count method that to it retrieve a value 
// We need to use the get_result method and pass the expected result to it in our case a i64
pub fn get_users_total_count(connection: &PgConnection) -> i64{
    let user_total_count = users::table()
        .count()
        .get_result::<i64>(connection)
        .expect("Error While Trying to Query Users totalcount."); // Here we expect an Error in case it panic we see why and where
    return user_total_count
}

// Here we can get a user by its email, 
// Since the user email is a unique constraint the behavior that we get is 1 User == 1 Email
// Based on that information, we can query the users that are not removed cause if they are removed = true
// They should not appear in our queries by convention :D
// And Filter a record based on the user email that we are passing
// And to finish we call the first method and pass the expected return in our case the queriable User struct
// And for advanced validation pourposes we call the .optional method to say that we may not recive a User
// To finish things we call the expect method just in case something goes wrong
pub fn get_user_by_email(connection: &PgConnection, user_email: &str) -> Option<User>{
    let user = users::table()
        .filter(email.eq(user_email))
        .filter(removed.eq(false))
        .first::<User>(connection)
        .optional()
        .expect("User not found whit argument email."); // Here we expect an Error in case it panic we see why and where
    return user
}

// In this query we will return all the users not removed in our Database
// We want to return all the users and the struct used for it is a vector populated with our users
// So we use the load method to specify our return value, and passing the type of load that we want
// In this case we want a load of user (a vector of users or: Vec<User>)
// And to finish things a expect to handle any error that can occur
pub fn get_all_db_users_with_filters(connection: &PgConnection) -> Vec<User>{
    let db_users = users::table()
        .filter(removed.eq(false))
        .load::<User>(connection)
        .expect("Error While trying to get users"); // Here we expect an Error in case it panic we see why and where
    return db_users
}