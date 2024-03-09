
use chesto::root_display::*;
use crate::screens::*;

use self::screens::MainScreen;

pub struct MainDisplay {
    pub root_display: BaseRootDisplay,
}

impl MainDisplay {
    pub fn new() -> MainDisplay {
        let mut root_display = BaseRootDisplay::new(800, 600);

        root_display.input_events.quit_action = Some(MainDisplay::quit);
        root_display.set_screen(Box::new(MainScreen::new()));

        MainDisplay {
            root_display
        }
    }

    pub fn main_loop(&mut self) {
        self.root_display.main_loop();
    }

    pub fn quit() {
        std::process::exit(0);
    }
}