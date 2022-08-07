#![allow(unused)]
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let point: Point = Point { x: 10, y: 15 };
    println!("{point:?}");
    let serialized: String = serde_json::to_string(&point).unwrap();
    println!("{serialized}");
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("{deserialized:?}");

    // get the response as a String
    let todos_text: String = Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .text()
        .await?;

    println!("{todos_text}");

    // get the response as a vector of Todo structs
    let todos_json: Vec<Todo> = Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{todos_json:#?}");

    // send a new Todo
    let new_todo = Todo {
        user_id: 7,
        id: None,
        title: "Things to do".to_string(),
        completed: false,
    };

    let new_todo: Todo = Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{new_todo:#?}");

    // generate a JSON object
    let json: Value = json!({
      "userId": 1,
      "id": 16,
      "title": "accusamus eos facilis sint et aut voluptatem",
      "completed": true
    });

    println!("{json:#?}");

    let new_todo: Todo = Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&json)
        .send()
        .await?
        .json()
        .await?;

    println!("{new_todo:#?}");

    Ok(())
}
