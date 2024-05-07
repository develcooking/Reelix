slint::include_modules!();


fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    //count upwards if button is pressed
   /* ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });*/

    //will implement link opening
   /* Tabsandmore.on_open_url(|url| {
        open::that(url.as_str()).ok();
    });*/
    ui.run()
}
