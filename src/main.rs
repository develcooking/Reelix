slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    ui.on_hello_value({
    move |hello_value|{
        println! ("bool: {}", hello_value);
    }});

    ui.run()
}