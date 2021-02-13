use diesel::{self, prelude::*};

mod schema {
    table! {
        users {
            id -> Nullable<Integer>,
            name -> Text,
            surname -> Text,
            email -> Text,
            pw -> Text,
            role -> Text,
        }
    }
}

use self::schema::users;
use self::schema::users::dsl::{users as all_users};

#[table_name="users"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, FromForm)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub pw: String,
    pub role: String
}

impl User {
    pub fn all(conn: &SqliteConnection) -> Vec<User> {
        all_users.order(users::id.desc()).load::<User>(conn).unwrap()
    }

    pub fn insert(id: Option<i32>, name: String, surname: String, email: String, pw: String, role: String, conn: &SqliteConnection) -> bool {
        let u = User { id, name, surname, email, pw, role };
        diesel::insert_into(users::table).values(&u).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_users.find(id)).execute(conn).is_ok()
    }

    pub fn edit_with_id(id: Option<i32>, name: String, surname: String, email: String, pw: String, role: String, conn: &SqliteConnection) -> bool {
        /*let updated_row = diesel::update(all_users.find(id))
            .set((name.eq(&name), surname.eq(&surname), email.eq(&email), role.eq(&role)))
            .get_result(conn);*/
        let u = User { id, name, surname, email, pw, role };
        diesel::replace_into(users::table).values(&u).execute(conn).is_ok()
        //return updated_row.is_ok()
    }
}
