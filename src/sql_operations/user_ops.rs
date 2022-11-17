use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::models::NewUser;
use crate::models::User;
use crate::schema::users::dsl::*;

pub fn add_user(user: &NewUser, connection: &PgConnection){
    diesel::insert_into(users)
    .values(user)
    .execute(connection)
    .expect("Deu merda mané, não inseriu o usuário");
}

pub fn get_users_total_count(connection: &PgConnection) -> i64{
    let user_total_count = users::table()
        .count()
        .get_result::<i64>(connection)
        .expect("Error While Trying to Query Users totalcount.");
    return user_total_count
}

pub fn get_user_by_email(connection: &PgConnection, user_email: &str) -> Option<User>{
    let user = users::table()
        .filter(email.eq(user_email))
        .first::<User>(connection)
        .optional()
        .expect("User not found whit argument email.");
    return user
}