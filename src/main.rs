mod screens;
mod main_display;
use main_display::MainDisplay;

pub fn main() -> Result<(), String> {
    println!("Starting main");

    let mut display = MainDisplay::new();

    println!("MainDisplay created");
    print!("{}", std::env::consts::OS);

    display.main_loop();
    
    Ok(())
}
