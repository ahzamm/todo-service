use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("select * from todo_list order by id desc").await.unwrap();
    let todo = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todo)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id desc")
        .await
        .unwrap();
    let todo = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todo)
}
