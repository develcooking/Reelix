use mysql::*;
use mysql::prelude::*;
slint::include_modules!();
use lazy_static::lazy_static;

#[derive(Debug)]
struct Typ {
    typ: String,
}

// Define a connection pool to the MySQL database
lazy_static! {
    static ref POOL: Pool = {
        let database_url: &str = "mysql://theuser:12345@141.13.222.38:3306/thebanking1";
        Pool::new(database_url)
            .expect("Failed to create a connection pool")
    };
}



// Function to retrieve data from the database and return it as a formatted string
fn askfordata() -> String {
    let mut output_text = String::new();

    // Get a connection from the connection pool
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");

    // Query the database to fetch data and map it to a vector of Typ structs
    let types: Vec<Typ> = conn.query_map(
        r"SELECT * FROM Typtabelle ORDER BY Typen ASC;",
        |typ| Typ { typ },
    ).expect("Failed to fetch data");

    // Format the retrieved data into a string
    for typ in types {
        output_text.push_str(&format!("{}\n", typ.typ));
    }
    println!("Data retrieved successfully");
    output_text
}



// Function to insert data into the database after filtering out unwanted characters
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



// Function to update the display in the UI with data from the database
fn update_database_display(ui: &MainWindow) {
    let output_text: String = askfordata();
    ui.set_thedatabase(output_text.into());
}



// Main function to run the UI and handle user interactions
fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    // Update the UI display with data from the database
    update_database_display(&ui);

    let ui_handle_copy = ui_handle.clone();

    // Define a closure to update the UI display when requested by the user
    ui.on_show_database({
        move || {
            update_database_display(&ui_handle_copy.unwrap());
        }
    });

    // Define a closure to create and insert data into the database based on user input
    ui.on_createtype(move |newtypeinput| {
        let extract_string: String = newtypeinput.trim().parse().unwrap();
        createdata(&extract_string);
        
        update_database_display(&ui_handle.unwrap());
    });

    ui.run()
}
