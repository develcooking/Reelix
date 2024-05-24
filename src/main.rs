use slint::{SharedString, VecModel};
slint::include_modules!();
use std::rc::Rc;
use std::cell::RefCell;
use mysql::*;
use mysql::prelude::*;
use lazy_static::lazy_static;
use chrono::{Local, Timelike, Datelike};

lazy_static! {
    static ref POOL: Pool = {
        let database_url: &str = "mysql://theuser:12345@141.13.222.38:3306/thebanking1";
        Pool::new(database_url)
            .expect("Failed to create a connection pool")
    };
}

// Struct to hold both bool value and input string
#[derive(Debug)]
struct DataBundle {
    bool_value: Option<bool>,
    input_string: Option<String>,
}

#[derive(Debug)]
struct Databundlesendreq {
    comment_string: Option<String>,
    current_value_type: Option<String>,
    current_location: Option<String>
}

// Struct to hold database records
#[derive(Debug)]
struct DatabaseRecord {
    typen: String,
    osdep: Option<bool>,
}

fn fetch_data_from_database() -> Result<Vec<DatabaseRecord>, slint::PlatformError> {
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");
    let records: Vec<DatabaseRecord> = conn
        .query_map(
            "SELECT Typen, osdep FROM Typtabelle ORDER BY Typen ASC",
            |row: Row| {
                let typen: String = row.get("Typen").unwrap();
                let osdep: Option<bool> = row.get("osdep").unwrap();
                DatabaseRecord { typen, osdep }
            },
        )
        .map_err(|err| slint::PlatformError::from(format!("MySQL error: {}", err)))?;

    Ok(records)
}

fn remove_data_from_database(valueofcombobox: &str) {
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");
    let typen = valueofcombobox.trim();
    if !typen.is_empty() && typen != "Select a Category to Remove" {
        conn.exec_drop(
            r"DELETE FROM Typtabelle WHERE Typen = ?",
            (&typen,)
        ).expect("Error while removing a Category");
    
        println!("Data removed successfully!");
    } else {
        println!("Default String to Remove or empty string can't be removed");
    }
}

fn update_database_display(ui: &MainWindow) -> Result<(), slint::PlatformError> {
    let data_from_db = fetch_data_from_database()?;
    let mut shared_typen_strings = Vec::new();
    let mut shared_osdep_strings = Vec::new(); // Proper initialization of shared_osdep_strings

    for record in data_from_db {
        shared_typen_strings.push(SharedString::from(record.typen.clone()));
        // Extract the value from record.osdep and add it to shared_osdep_strings
        if let Some(osdep) = record.osdep {
            shared_osdep_strings.push(osdep);
        }
    }

    let model_rc = Rc::new(VecModel::from(shared_typen_strings)).into();

    // Print each element in shared_osdep_strings
    for osdep in &shared_osdep_strings {
        println!("Checkbox value: {}", osdep);
    }

    ui.set_the_model(model_rc);
    let current_datetime = Local::now();
    let date = current_datetime.format("%Y-%m-%d").to_string();
    let shared_date = SharedString::from(date);
    ui.set_thedate(shared_date.into());

    Ok(())
}

fn sendrequest(data_bundle_sendreq: &Databundlesendreq) {
    // Debug output for comment_string, current_value_type, and current_location
    println!("sendrequest: Received comment_string: {:?}", data_bundle_sendreq.comment_string);
    println!("sendrequest: Received current_value_type: {:?}", data_bundle_sendreq.current_value_type);
    println!("sendrequest: Received current_location: {:?}", data_bundle_sendreq.current_location);

    // Get a connection from the connection pool
    let mut conn = POOL.get_conn().expect("Failed to get a connection from the pool");

    if let Some(current_value_type) = &data_bundle_sendreq.current_value_type {
        let current_datetime = Local::now();
        // extract Date
        let year = current_datetime.year();
        let month = current_datetime.month();
        let day = current_datetime.day();

        // extract Time
        let hour = current_datetime.hour();
        let minute = current_datetime.minute();
        let second = current_datetime.second();
        let date = format!("{}-{}-{}", year, month, day);
        let time = format!("{}:{}:{}", hour, minute, second);
        let operating_system = "Windows";

        // Check if the comment is None, empty or contains only whitespace
        let comment_log = data_bundle_sendreq
            .comment_string
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| "NONE".to_string());

        // Use current_location from data_bundle_sendreq
        let location = data_bundle_sendreq
            .current_location
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| "UNKNOWN".to_string()); // Default to "UNKNOWN" if location is None or empty

        conn.exec_drop("INSERT INTO Requests (Date, Time, Type, Operating_System, Comment_Log, Location) VALUES (?, ?, ?, ?, ?, ?)",
            (date, time, current_value_type, operating_system, comment_log, location)).unwrap();
    } else {
        println!("current_value_type is None; cannot insert into database.");
    }
}

fn createdata(data_bundle: &DataBundle) {
    // Get a connection from the connection pool
    let mut conn = POOL.get_conn().expect("Failed to get a connection from the pool");

    // Insert the data into the database if input_string is valid
    if let Some(input_string) = &data_bundle.input_string {
        let filtered_text = input_string.trim().to_string();
        println!("Original String: '{}'", input_string);
        println!("Received bool_value: {:?}", data_bundle.bool_value);
        println!("Filtered String: '{}'", filtered_text);

        if let Some(true) | Some(false) = data_bundle.bool_value {
            // Insert data into database
            if !filtered_text.is_empty() {
                conn.exec_drop(
                    r"INSERT INTO Typtabelle (Typen, osdep) VALUES (?, ?)",
                    (&filtered_text, data_bundle.bool_value), // Explicitly use true
                ).expect("Error inserting data");

                println!("Data inserted successfully!");
            } else {
                println!("Empty or whitespace-laden type string not inserted into the database.");
            }
        } else {
            println!("No bool value provided or bool value is None, skipping data insertion.");
        }
    } else {
        println!("Input string is None; cannot insert into database.");
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();
    let _ = update_database_display(&ui);
    
    // Rc and RefCell for safe access and mutation of the data
    let data_bundle = Rc::new(RefCell::new(DataBundle {
        bool_value: Some(false),
        input_string: None,
    }));

    // Rc and RefCell for safe access and mutation of the data
    let data_bundle_sendreq = Rc::new(RefCell::new(Databundlesendreq {
        comment_string: None,
        current_value_type: None,
        current_location: None,
    }));

    let ui_handle_copy = ui_handle.clone();
    let ui_handle_copy2 = ui_handle.clone();
    let ui_handle_copy3 = ui_handle.clone();

    // Integration of ui.on_ossupport_value into event handling
    let ossupport_data_bundle = Rc::clone(&data_bundle);
    ui.global::<Logic>().on_ossupport_value(move |ossupport_value: bool| {
        // Update bool_value in the shared data bundle
        ossupport_data_bundle.borrow_mut().bool_value = Some(ossupport_value);

        println!("Received ossupport_value: {}", ossupport_value);
    });

    // Integration of ui.on_createtype into event handling
    let createtype_data_bundle = Rc::clone(&data_bundle);
    ui.global::<Logic>().on_createtype(move |newtypeinput: SharedString| {
        // Update input_string in the shared data bundle
        createtype_data_bundle.borrow_mut().input_string = Some(newtypeinput.to_string());
        // Call createdata if both bool_value and input_string are available
        let data_bundle_ref = createtype_data_bundle.borrow();
        if let Some(true) | Some(false) = data_bundle_ref.bool_value {
            createdata(&data_bundle_ref);
        }

        let _ = update_database_display(&ui_handle_copy.unwrap());
    });

    // Define a closure to create and insert data into the database based on user input
    ui.global::<Logic>().on_cabavalueofcombobox(move |valueofcombobox: SharedString| {
        println!("removed {} from database", valueofcombobox);
        remove_data_from_database(&valueofcombobox);
        let _ = update_database_display(&ui_handle_copy2.unwrap());
    });

    let commentrequest_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_commentrequest(move |comment: SharedString| {
        commentrequest_data_bundle.borrow_mut().comment_string = Some(comment.to_string());
        println!("value of comment is: {}", comment);
    });

    let makerecord_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_makerecord(move |current_value: SharedString| {
        // Update current_value_type and ensure comment_string and current_location are set before calling sendrequest
        {
            let mut bundle = makerecord_data_bundle.borrow_mut();
            bundle.current_value_type = Some(current_value.to_string());
            println!("value of record is: {}", current_value);
        }
        sendrequest(&makerecord_data_bundle.borrow());
    });

    let location_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_location(move |location: SharedString| {
        {
            let mut bundle = location_data_bundle.borrow_mut();
            bundle.current_location = Some(location.to_string());
            println!("value of location is: {}", location);
        }
    });

    ui.global::<Logic>().on_open_url(|url: SharedString| {
        open::that(url.as_str()).ok();
    });

    ui.run()?;
    Ok(())
}
