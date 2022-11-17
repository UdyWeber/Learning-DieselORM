use diesel::PgConnection;

use self::users_load::data_load_users;

pub mod users_load;

pub fn data_load_all(connection: &PgConnection){
    data_load_users(connection);
}