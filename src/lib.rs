#![allow(non_snake_case)]

use asr::{
    future::next_tick, 
    settings::Gui, 
    Process, 
    watcher::Watcher,
    PointerSize::Bit64,
    Address,
    print_message,
    timer::{
        reset, 
        set_game_time, 
        set_variable, 
        set_variable_float, 
        set_variable_int, 
        split, 
        start, 
        state, 
        TimerState,
        pause_game_time,
    },
};
mod splitter_settings;
mod offsets;
use crate::offsets::get_offsets;

asr::async_main!(stable);

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = splitter_settings::Settings::register();

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
            print_message("invalid plattform");
            process_name = "";
        }
    }

    
    print_message("Setup done. Waiting for Process.");

    loop {
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                print_message("Process found.");

                if let Ok(base_address) = process.get_module_address(process_name){
                    set_variable_int("base", base_address.value());
                    let offset_arrays = get_offsets(&process, process_name);


                    // Game Timer (seconds)
                    
                    let mut watch_fPlayTimeCleared: Watcher<f64> = Watcher::new();
                    watch_fPlayTimeCleared.update_infallible(0f64);

                    print_message("Starting Loop.");
                    loop {
                        settings.update();
                        
                        // TODO: Do something on every tick.

                        
                        // Game Timer
                        if let Ok(time) = process.read_pointer_path::<f64>(
                            base_address,
                            Bit64,
                            &offset_arrays.fPlayTimeCleared,
                        ) {
                            watch_fPlayTimeCleared.update_infallible(time);
                            set_variable_float("fPlayTimeCleared", time);
                            //set_game_time(Duration::seconds_f64(time));
                        }
                        
                        next_tick().await;
                    }
                }
            })
            .await;
    }
}
