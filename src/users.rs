/* To be able to return Templates */
use rocket_contrib::templates::Template;
use std::collections::HashMap;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

/* Database data structs (User, NewUser) */
use crate::models::*;

/* To be able to parse raw forms */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Flash message and redirect */
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

/* List our inserted users */
#[get("/")]
pub fn list(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    /* Get all our users from database */
    let users: Vec<User> = users::table
        .select(users::all_columns)
        .load::<User>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");

    /* Insert on the template rendering
    context our new users vec */
    if let Some(ref msg) = flash {
        context.insert("data", (users, msg.msg()));
    } else {
        context.insert("data", (users, "Listing users..."));
    }

    /* Return the template */
    Template::render("list", &context)
}

#[get("/new")]
pub fn new(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("new", context)
}

#[post("/insert", data = "<user_data>")]
pub fn insert(content_type: &ContentType, user_data: Data) -> Flash<Redirect> {
    /* File system */
    use std::fs;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("spotted_photo"),
        MultipartFormDataField::text("fantasy_name"),
        MultipartFormDataField::text("real_name"),
        MultipartFormDataField::text("strength_level"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, user_data, options);

    match multipart_form_data {
        Ok(form) => {
            /* If everything is ok, we will move the image and the insert into our datatabase */
            let user_img = match form.files.get("spotted_photo") {
                Some(img) => {
                    let file_field = &img[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;

                    /* Lets split name to get format */
                    let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

                    /* Path parsing */
                    let absolute_path: String = format!("imgs/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();

                    Some(format!("imgs/{}", _file_name.clone().unwrap()))
                }
                None => None,
            };

            /* Insert our form data inside our database */
            let insert = diesel::insert_into(users::table)
                .values(NewUser {
                    fantasy_name: match form.texts.get("fantasy_name") {
                        Some(value) => &value[0].text,
                        None => "No Name.",
                    },
                    real_name: match form.texts.get("real_name") {
                        Some(content) => Some(&content[0].text),
                        None => None,
                    },
                    spotted_photo: user_img.unwrap(),
                    strength_level: match form.texts.get("strength_level") {
                        Some(level) => level[0].text.parse::<i32>().unwrap(),
                        None => 0,
                    },
                })
                .execute(&crate::establish_connection());

            match insert {
                Ok(_) => Flash::success(
                    Redirect::to("/"),
                    "Success! We got a new User on our database!",
                ),
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!(
                        "Houston, We had problems while inserting things into our database ... {}",
                        err_msg
                    ),
                ),
            }
        }
        Err(err_msg) => {
            /* Falls to this patter if theres some fields that isn't allowed or bolsonaro rules this code */
            Flash::error(
                Redirect::to("/new"),
                format!(
                    "Houston, We have problems parsing our form... Debug info: {}",
                    err_msg
                ),
            )
        }
    }
}

#[get("/update/<id>")]
pub fn update(id: i32) -> Template {
    let mut context = HashMap::new();
    let user_data = users::table
        .select(users::all_columns)
        .filter(users::id.eq(id))
        .load::<User>(&crate::establish_connection())
        .expect("Something happned while retrieving the hero of this id");

    context.insert("user", user_data);

    Template::render("update", &context)
}

#[post("/update", data = "<user_data>")]
pub fn process_update(content_type: &ContentType, user_data: Data) -> Flash<Redirect> {
    /* File system */
    use std::fs;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("spotted_photo"),
        MultipartFormDataField::text("id"),
        MultipartFormDataField::text("fantasy_name"),
        MultipartFormDataField::text("real_name"),
        MultipartFormDataField::text("strength_level"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, user_data, options);

    match multipart_form_data {
        Ok(form) => {
            /* If everything is ok, we will move the image and the insert into our datatabase */
            let user_img = match form.files.get("spotted_photo") {
                Some(img) => {
                    let file_field = &img[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;

                    /* Lets split name to get format */
                    let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

                    /* Path parsing */
                    let absolute_path: String = format!("imgs/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();

                    Some(format!("imgs/{}", _file_name.clone().unwrap()))
                }
                None => None,
            };

            /* Insert our form data inside our database */
            let insert = diesel::update(
                users::table.filter(
                    users::id.eq(form.texts.get("id").unwrap()[0]
                        .text
                        .parse::<i32>()
                        .unwrap()),
                ),
            )
            .set(NewUser {
                fantasy_name: match form.texts.get("fantasy_name") {
                    Some(value) => &value[0].text,
                    None => "No Name.",
                },
                real_name: match form.texts.get("real_name") {
                    Some(content) => Some(&content[0].text),
                    None => None,
                },
                spotted_photo: user_img.unwrap(),
                strength_level: match form.texts.get("strength_level") {
                    Some(level) => level[0].text.parse::<i32>().unwrap(),
                    None => 0,
                },
            })
            .execute(&crate::establish_connection());

            match insert {
                Ok(_) => Flash::success(
                    Redirect::to("/"),
                    "Success! We got a new User on our database!",
                ),
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!(
                        "Houston, We had problems while inserting things into our database ... {}",
                        err_msg
                    ),
                ),
            }
        }
        Err(err_msg) => {
            /* Falls to this patter if theres some fields that isn't allowed or bolsonaro rules this code */
            Flash::error(
                Redirect::to("/new"),
                format!(
                    "Houston, We have problems parsing our form... Debug info: {}",
                    err_msg
                ),
            )
        }
    }
}

#[get("/delete/<id>")]
pub fn delete(id: i32) -> Flash<Redirect> {
    diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(&crate::establish_connection())
        .expect("Ops, we can't delete this.");
    Flash::success(Redirect::to("/"), "Yey! The hero was deleted.")
}