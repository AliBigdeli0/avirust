use config_ui::*;
 

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    println!("this application does not have native version");
    println!("config ui version is : {}" , get_version());
    Ok(())
}

#[cfg(target_arch="wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new() 
            .start(
                "canvas_id",
                web_options,
                Box::new(|cc| Box::new(AppTemplate::new(cc))),
            )
            .await
            .expect("failed to start eframe...");
    });
}
