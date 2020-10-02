#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::sql_query;
//
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use serde::{Deserialize, Serialize};
mod lib;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
use schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
//
//
pub fn get_content(filename: String ) -> String{
//    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

//    println!("With text:\n{}", contents);
    return contents;
}
#[derive(Serialize, Deserialize , Debug)]
struct TaskItem {
    id: i64,
    title: String,
    content: String,
}    
//
fn create_thread( items: Vec<TaskItem> ) -> JoinHandle<()>{
    use self::schema::posts::dsl::{id, posts};

    let handle = thread::spawn( move|| {
        let connection = lib::establish_connection();
        for row in &items {
            let s_title = &row.title;
            let s_content = &row.content;    
            let new_post = NewPost {
                title: s_title.to_string(),
                body: s_content.to_string(),
            };
            diesel::insert_into(posts)
                .values(&new_post)
                .execute(&connection)
                .expect("Error saving new post");     
        }
    });
    return handle;
}
//
fn exec_thread(items: Vec<TaskItem>) {
    use self::schema::posts::dsl::{id, posts};
    let tup = conver_array(items ,2);
//println!("len1={}" , tup.0.len() );
    let handle = create_thread( tup.0 );
    let handle_2 = create_thread(tup.1 );
    handle.join().unwrap();
    handle_2.join().unwrap();
}
//
fn conver_array(items: Vec<TaskItem>, thread_num : i64) ->(Vec<TaskItem>,Vec<TaskItem>){
    let mut count: usize = 1;
    let size = &items.len();
    let thread_num_big = thread_num as usize;
    let n1_max = size / thread_num_big;

//    println!("n1={}", n1 );
    let mut items_1 : Vec<TaskItem> = Vec::new();
    let mut items_2 : Vec<TaskItem> = Vec::new();
    for item in &items {
        let d = TaskItem { 
            id: item.id ,
            title: String::from(&item.title), 
            content: String::from(&item.content) 
        };
        if(count <= n1_max){
            items_1.push( d );
        }else{
            items_2.push( d );
        }
        count += 1;
    } 
    return (items_1, items_2)   
}
//
pub fn exec_main(json_fname: String){
        let json = get_content( json_fname );
//        println!("{}", json);
        let deserialized: Vec<TaskItem> = serde_json::from_str(&json).unwrap();
        exec_thread(deserialized);
}
//
fn main() {
    println!("#start-db");
    let fname = "/home/naka/work/node/express/app7/public/tasks.json";
    exec_main( fname.to_string() ); 
}
