use i_slint_backend_winit;
use i_slint_backend_winit::winit::window::WindowButtons;
use i_slint_backend_winit::WinitWindowAccessor;

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

// Define a structure to hold boolean value and input string
#[derive(Debug)]
struct DataBundleCreate {
    bool_value: Option<bool>,
    input_string: Option<String>,
}

// Define a structure to hold data for sending requests
#[derive(Debug)]
struct Databundlesendreq {
    comment_string: Option<String>,
    current_value_type: Option<String>,
    current_location: Option<String>,
    operating_system: Option<String>,
    datetime: Option<String>,
}

// Define a structure to represent a database record
#[derive(Debug)]
struct DatabaseRecord {
    typen: String,
    osdep: Option<bool>,
}

// Define a structure to hold checkbox data
#[derive(Debug, Default)]
struct CheckboxData {
    osdep_value: Option<bool>,
}

// Function to fetch data from the database
fn fetch_data_from_database() -> Result<Vec<DatabaseRecord>, slint::PlatformError> {
    // Establish a connection from the connection pool
    let mut conn = POOL.get_conn().expect("Failed to get a connection from the pool");
    // Execute a query to fetch data
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

// Function to execute a database query
fn execute_query(query: &str, params: &[&dyn ToValue]) -> Result<(), Error> {
    // Establish a connection from the connection pool
    let mut conn = POOL.get_conn().expect("Failed to get a connection from the pool");
    // Execute the query with parameters
    conn.exec_drop(query, params)?;
    Ok(())
}

// Function to ask for checkbox values from the database
fn ask_for_checkbox_values(sel: &str, checkbox_data: &Rc<RefCell<CheckboxData>>) {
    // Formulate a query to select checkbox values based on selection
    let query = format!("SELECT osdep FROM Typtabelle WHERE Typen = '{}'", sel);
    // Execute the query and map the results
    let result: Vec<Option<bool>> = POOL.get_conn().unwrap().query_map(&query, |osdep: Option<i32>| osdep.map(|val| val != 0)).unwrap();

    // Extract and update the checkbox value
    let value = result.get(0).cloned().unwrap_or_default();
    checkbox_data.borrow_mut().osdep_value = value;
    println!("Osdepvalue of {} is: {:?}", sel, value);
}

// Function to remove data from the database
fn remove_data_from_database(valueofcombobox: &str) {
    // Establish a connection from the connection pool
    let mut conn = POOL.get_conn()
        .expect("Failed to get a connection from the pool");
    let typen = valueofcombobox.trim();
    if !typen.is_empty() && typen != "Select a Category to Remove" {
        // Execute a query to delete data from the database
        conn.exec_drop(
            r"DELETE FROM Typtabelle WHERE Typen = ?",
            (&typen,)
        ).expect("Error while removing a Category");
    
        println!("Data removed successfully!");
    } else {
        println!("Default String to Remove or empty string can't be removed");
    }
}

fn update_database_display(ui: &MainWindow, checkbox_data: &Rc<RefCell<CheckboxData>>) -> Result<(), slint::PlatformError> {
    let data_from_db = fetch_data_from_database()?;
    let mut shared_typen_strings = Vec::new();
    let mut shared_osdep_strings = Vec::new();

    // Process database records and prepare data for UI display
    for record in data_from_db {
        shared_typen_strings.push(SharedString::from(record.typen.clone()));
        // Extract the value from record.osdep and add it to shared_osdep_strings
        if let Some(osdep) = record.osdep {
            shared_osdep_strings.push(osdep);
        }
    }

    // Create Vecmodel for Comboxes
    let model_rc = Rc::new(VecModel::from(shared_typen_strings)).into();

    // Update UI with model data and checkbox values
    if let Some(osdep_value) = checkbox_data.borrow().osdep_value {
        println!("Shared osdep value: {}", osdep_value);
        ui.set_ossupportbox_value(osdep_value);
    }
    ui.set_the_model(model_rc);
    let current_datetime = Local::now();
    let date = current_datetime.format("%Y-%m-%d").to_string();
    let shared_date = SharedString::from(date);
    ui.set_thedate(shared_date.into());

    Ok(())
}

fn send_request(data_bundle_sendreq: &Databundlesendreq) {
    println!("sendrequest: {:?}", data_bundle_sendreq);
    if let (Some(current_value_type), Some(operating_system)) = (&data_bundle_sendreq.current_value_type, &data_bundle_sendreq.operating_system) {
        let current_datetime = Local::now();
        let date = current_datetime.format("%Y-%m-%d").to_string();
        let datetime = &data_bundle_sendreq.datetime;
        let comment_log = data_bundle_sendreq.comment_string.as_ref().map(|s| s.trim()).unwrap_or_default().to_string();
        let location = data_bundle_sendreq.current_location.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or_else(|| "UNKNOWN");
        let query = "INSERT INTO Requests (Date, Time, Type, Operating_System, Comment_Log, Location) VALUES (?, ?, ?, ?, ?, ?)";
        execute_query(query, &[&date, &datetime, current_value_type, operating_system, &comment_log, &location]).unwrap();
    } else {
        println!("current_value_type is None; cannot insert into database.");
    }
}

fn createcategory(data_bundle: &DataBundleCreate) {
    // Insert the data into the database if input_string is valid
    if let Some(input_string) = &data_bundle.input_string {
        let filtered_text = input_string.trim().to_string();
        println!("Original String: '{}'", input_string);
        println!("Received bool_value: {:?}", data_bundle.bool_value);
        println!("Filtered String: '{}'", filtered_text);
        if let Some(bool_value) = data_bundle.bool_value {
            if bool_value == true || bool_value == false {
                let checkboxvalue = !bool_value;
            // Insert data into database
                if !filtered_text.is_empty() {
                    let query = "INSERT INTO Typtabelle (Typen, osdep) VALUES (?, ?)";
                    execute_query(query, &[&filtered_text, &checkboxvalue]).expect("Error inserting data");
                    println!("Data inserted successfully!");
                } else {
                    println!("Empty or whitespace-laden type string not inserted into the database.");
                }
            } else {
                println!("No bool value provided or bool value is None, skipping data insertion.");
            }
        }
    } else {
        println!("Input string is None; cannot insert into database.");
    }
}


fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    ui.window().with_winit_window(|winit_win| {
        winit_win.set_enabled_buttons(WindowButtons::MINIMIZE | WindowButtons::CLOSE);
    });
    let ui_handle = ui.as_weak();
    let checkbox_data = Rc::new(RefCell::new(CheckboxData::default()));
    let _ = update_database_display(&ui, &checkbox_data);
    
    // Rc and RefCell for safe access and mutation of the data
    let data_bundle_create = Rc::new(RefCell::new(DataBundleCreate {
        bool_value: Some(false),
        input_string: None,
    }));

    let data_bundle_sendreq = Rc::new(RefCell::new(Databundlesendreq {
        comment_string: None,
        current_value_type: None,
        current_location: None,
        operating_system: None,
        datetime: None,
    }));

    let ui_handle_copy = ui_handle.clone();
    let ui_handle_copy2 = ui_handle.clone();
    let ui_handle_copy3 = ui_handle.clone();
    let checkbox_data_copy1 = Rc::clone(&checkbox_data);
    let checkbox_data_copy2 = Rc::clone(&checkbox_data);

    let ossupport_data_bundle = Rc::clone(&data_bundle_create);
    ui.global::<Logic>().on_ossupport_value(move |ossupport_value: bool| {
        ossupport_data_bundle.borrow_mut().bool_value = Some(ossupport_value);
        println!("Received ossupport_value: {}", ossupport_value);
    });

    let createtype_data_bundle = Rc::clone(&data_bundle_create);
    ui.global::<Logic>().on_createtype(move |newtypeinput: SharedString| {
        createtype_data_bundle.borrow_mut().input_string = Some(newtypeinput.to_string());
        let data_bundle_ref = createtype_data_bundle.borrow();
        if let Some(true) | Some(false) = data_bundle_ref.bool_value {
            createcategory(&data_bundle_ref);
        }

        let _ = update_database_display(&ui_handle_copy.unwrap(), &checkbox_data_copy1);
    });

    let commentrequest_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_commentrequest(move |comment: SharedString| {
        commentrequest_data_bundle.borrow_mut().comment_string = Some(comment.to_string());
        println!("value of comment is: {}", comment);
    });

    let makerecord_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_makerecord(move |current_value: SharedString| {
        {
            let mut bundle = makerecord_data_bundle.borrow_mut();
            bundle.current_value_type = Some(current_value.to_string());
            println!("value of record is: {}", current_value);
        }
        send_request(&makerecord_data_bundle.borrow());
    });

    let location_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_location(move |location: SharedString| {
        {
            let mut bundle = location_data_bundle.borrow_mut();
            bundle.current_location = Some(location.to_string());
            println!("value of location is: {}", location);
        }
    });

    let operating_system_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_OperatingSystem(move |os: SharedString| {
        {
            let mut bundle = operating_system_data_bundle.borrow_mut();
            bundle.operating_system = Some(os.to_string());
            println!("value of operating system is: {}", os);
        }
    });

    let datetime_data_bundle = Rc::clone(&data_bundle_sendreq);
    ui.global::<Logic>().on_datetime(move |datetime: SharedString| {

        let timeformatted = if datetime == "Now" {
            Local::now().format("%H:%M:%S").to_string()
        } else if datetime == "Morning" {
            String::from("8:01:00")
        } else if datetime == "Afternoon" {
            String::from("13:01:00")
        } else {
            String::from("Unknown")
        };

        let mut bundle = datetime_data_bundle.borrow_mut();
        bundle.datetime = Some(timeformatted.to_string());
        println! ("datetime is: {}", timeformatted)
    });

    // If the callback is trigergt grabs the current value of the comboboxScroll and delets it
    ui.global::<Logic>().on_cabavalueofcombobox(move |valueofcombobox: SharedString| {
        println!("removed {} from database", valueofcombobox);
        remove_data_from_database(&valueofcombobox);
        let _ = update_database_display(&ui_handle_copy2.unwrap(), &checkbox_data_copy2);//updates the avalible values in the database
    });

    ui.global::<Logic>().on_currentselrecord(move |sel: SharedString| {
        {
            println!("value of combox is: {}", sel);
            ask_for_checkbox_values(&sel.to_string(), &checkbox_data);
            let _ = update_database_display(&ui_handle_copy3.unwrap(), &checkbox_data);
        }
    });

    // to open links in the web
    ui.global::<Logic>().on_open_url(|url: SharedString| {
        open::that(url.as_str()).ok();
    });

    ui.run()?;
    Ok(())
}