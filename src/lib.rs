pub mod convert;

#[cfg(target_os = "android")]
mod ui;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let options = eframe::NativeOptions {
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "lengthcal",
        options,
        Box::new(|_cc| Ok(Box::new(ui::LengthCal::default()))),
    );
}
