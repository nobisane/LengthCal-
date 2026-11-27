pub mod convert;

#[cfg(target_os = "android")]
slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    use slint::ComponentHandle;

    slint::android::init(app).expect("Android setup failed");
    let window = AppWindow::new().expect("window creation failed");

    let weak = window.as_weak();
    window.on_value_edited(move |from, text| {
        let Some(window) = weak.upgrade() else { return };
        let from = from as usize;
        let values = convert::convert_all(&text, from);
        let setters: [fn(&AppWindow, slint::SharedString); 8] = [
            AppWindow::set_km,
            AppWindow::set_m,
            AppWindow::set_yd,
            AppWindow::set_ft,
            AppWindow::set_inch,
            AppWindow::set_cm,
            AppWindow::set_mm,
            AppWindow::set_mi,
        ];
        // Never overwrite the box being typed in.
        for (i, set) in setters.iter().enumerate() {
            if i != from {
                let value = values.as_ref().map_or("", |v| v[i].as_str());
                set(&window, value.into());
            }
        }
    });

    window.run().expect("app stopped with an error");
}
