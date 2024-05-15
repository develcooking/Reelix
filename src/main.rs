use slint::{SharedString, VecModel};
slint::include_modules!();
use std::rc::Rc;
use mysql::*;
use mysql::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref POOL: Pool = {
        let database_url: &str = "mysql://theuser:12345@141.13.222.38:3306/thebanking1";
        Pool::new(database_url)
            .expect("Failed to create a connection pool")
    };
}

fn fetch_data_from_database() -> Result<Vec<String>, slint::PlatformError> {
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");
    let types: Vec<String> = conn
        .query_map(
            "SELECT Typen FROM Typtabelle ORDER BY Typen ASC",
            |row: Row| row.get("Typen").unwrap(),
        )
        .map_err(|err| slint::PlatformError::from(format!("MySQL error: {}", err)))?;

    Ok(types)
}

fn remove_data_from_database(valueofcombobox: &str) {
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");
    let typen = valueofcombobox.trim();
    if !typen.is_empty() && typen != "Select a Category to Remove" {
        conn.exec_drop(
            r"DELETE FROM Typtabelle WHERE Typen = ?",
            (&typen,)
        ).expect("Error while removeing a Category");
    
        println!("Data removed successfully!");
    } else {
        println!("Default String to Remove or empty string can't be removed");
    }

}

fn update_database_display(ui: &MainWindow) -> Result<(), slint::PlatformError> {
    let data_from_db = fetch_data_from_database()?;

    let mut shared_strings = Vec::new();
    for typ in data_from_db {
        shared_strings.push(SharedString::from(typ));
    }

    let model_rc = Rc::new(VecModel::from(shared_strings)).into();
    ui.set_the_model(model_rc);

    Ok(())
}

fn createdata(input_text: &str) {
    let trimmed_text = input_text.trim();
    let filtered_text = trimmed_text.chars()
        .collect::<String>();

    // Get a connection from the connection pool
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");

    println!("Original String: '{}'", input_text);
    println!("Filtered String: '{}'", filtered_text);

    // Insert the filtered text into the database if it's not empty
    if !filtered_text.is_empty() {
        conn.exec_drop(
            r"INSERT INTO Typtabelle (Typen) VALUES (?)",
            (&filtered_text,)
        ).expect("Error inserting data");

        println!("Data inserted successfully!");
    } else {
        println!("Empty or whitespace-laden type string not inserted into the database.");
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();
    let _ = update_database_display(&ui);
    

    let ui_handle_copy = ui_handle.clone();

    // Define a closure to create and insert data into the database based on user input
    ui.global::<Logic>().on_cabavalueofcombobox(move |valueofcombobox: SharedString| {
        println!("removed {} from database", valueofcombobox);
        remove_data_from_database(&valueofcombobox);
        let _ = update_database_display(&ui_handle_copy.unwrap());
    });

    ui.global::<Logic>().on_createtype(move |newtypeinput: SharedString| {
        let extract_string: String = newtypeinput.trim().parse().unwrap();
        createdata(&extract_string);

        let _ = update_database_display(&ui_handle.unwrap());
    });

    ui.global::<Logic>().on_open_url(|url: SharedString| {
        open::that(url.as_str()).ok();
    });
    

    ui.run()?;
    Ok(())
}
