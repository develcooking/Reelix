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

fn main() -> Result<(), slint::PlatformError> {
    let data_from_db = fetch_data_from_database()?;

    let mut shared_strings = Vec::new();
    for typ in data_from_db {
        shared_strings.push(SharedString::from(typ));
    }

    let ui = MainWindow::new()?;
    let model_rc = Rc::new(VecModel::from(shared_strings)).into();
    ui.set_the_model(model_rc);

    // Define a closure to create and insert data into the database based on user input
    ui.on_cabavalueofcombobox(move |valueofcombobox| {
       println!("hi {}", valueofcombobox)
    });


    ui.run()?;
    Ok(())
}