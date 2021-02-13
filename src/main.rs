#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod user;

use rocket::{Rocket};
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use diesel::SqliteConnection;

use crate::user::{User};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, users: Vec<User> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), users: User::all(conn)}
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, users: User::all(conn)}
    }
}

#[post("/", data = "<user_form>")]
fn new(user_form: Form<User>, conn: DbConn) -> Flash<Redirect> {
    let user = user_form.into_inner();
    if user.name.is_empty() || user.surname.is_empty() || user.email.is_empty() || user.pw.is_empty() || user.role.is_empty() {
        Flash::error(Redirect::to("/"), "Empty User cannot be created.")
    } else if User::insert(user.id, user.name, user.surname, user.email, user.pw, user.role , &conn) {
        Flash::success(Redirect::to("/"), "User successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[post("/edit?<user..>")]
fn edit(user: Option<Form<User>>, conn: DbConn) -> Result<Redirect, Template> {
    let user = user.unwrap().into_inner();
    if User::edit_with_id(user.id, user.name, user.surname, user.email, user.pw, user.role, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't toggle task.")))
    }
}

#[post("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if User::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "User was deleted."))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't delete task.")))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage<'_, '_>>, conn: DbConn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index, edit])
        .mount("/user", routes![new, delete])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
