use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await
        .unwrap();
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
        .prepare(
            "SELECT *
        FROM todo_item
        WHERE list_id = $1
        ORDER BY CASE WHEN checked = true THEN 1 ELSE 0 END, id DESC",
        )
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

pub async fn create_todo(client: &Client, title: String) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1)")
        .await
        .unwrap();
    let todo = client
        .query(&statement, &[&title])
        .await
        .expect("Error creating new list")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todo)
}

pub async fn create_todo_item(
    client: &Client,
    title: &String,
    list_id: &i32,
) -> Result<(), io::Error> {
    let statement = client
        .prepare("INSERT INTO todo_item (title, checked, list_id) VALUES ($1, $2, $3)")
        .await
        .unwrap();
    client
        .query(&statement, &[&title, &false, &list_id])
        .await
        .expect("Error creating new item");

    Ok(())
}

pub async fn todo_item_checked(
    client: &Client,
    id: &i32,
    checked: bool,
) -> Result<Option<i32>, io::Error> {
    let statement = client
        .prepare("UPDATE todo_item SET checked = $1 WHERE id = $2 returning list_id")
        .await
        .unwrap();
    let rows = client
        .query(&statement, &[&checked, &id])
        .await
        .expect("Error creating new item");

    if let Some(row) = rows.iter().next() {
        let list_id: i32 = row.get("list_id");
        Ok(Some(list_id))
    } else {
        Ok(None)
    }
}
