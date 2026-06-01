use asr::{future::next_tick, settings::Gui, Process};

asr::async_main!(stable);

#[derive(Gui)]
struct Settings {
    /// My Setting
    #[default = true]
    my_setting: bool,
    // TODO: Change these settings.
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    // Base Settings
    let plattform = "linux";
    let process_name: &str;
    match plattform {
        "linux" => {
            process_name = "MinaTheHollower";
        }
        "windows" => {
            process_name = "MinaTheHollower.exe";
        }
        _ => {
            asr::print_message("invalid plattform");
            process_name = "";
        }
    }

    asr::print_message("Setup done. Waiting for Process.");

    loop {
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                asr::print_message("Process found.");
                // TODO: Load some initial information from the process.
                asr::print_message("Starting Loop.");
                loop {
                    settings.update();
                    
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
