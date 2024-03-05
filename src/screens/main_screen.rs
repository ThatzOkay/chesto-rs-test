use chesto::{Element, ImageElement, InputEvent, Screen};
use chesto::elements::TextElement;
use sdl2::pixels::Color;

use super::second_screen::SecondScreen;

#[derive(Clone)]
pub struct MainScreen {
    elements: Vec<Box<dyn Element>>,
    next_screen: Option<Box<dyn Screen>>,
}


impl MainScreen {
    pub fn new() -> Self {
        MainScreen {
            elements: vec![
                Box::new(TextElement::new("This is a test", 1, None)),
                Box::new(TextElement {
                    text: "This is another test.".to_string(), size: 1, color: None, rendered: false
                }),
                Box::new(ImageElement {
                    src: "/path/to/image.png".to_string(),
                    color: Some(Color::RGB(252, 186, 3)),
                    rendered: false,
                })
            ],
            next_screen: None,
        }
    }
}

impl Screen for MainScreen {
    
    fn elements(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.elements
    }

    fn process(&mut self, input_events: InputEvent) -> bool {
        // own processing to override the base
        for element in self.elements() {
            element.process(input_events.clone());
        }

        if self.elements.iter().all(|element| element.is_rendered()) {
            std::thread::sleep(std::time::Duration::from_secs(5));
            self.set_screen(Box::new(SecondScreen::new()));
        }

        true
    }

    fn next_subscreen(&self) -> Option<Box<dyn Screen>> {
        if self.next_screen.is_none() {
            return None;
        }

        let next_screen = self.next_screen.as_deref().take();
        Some(dyn_clone::clone_box(&*next_screen.unwrap()))
    }

    fn set_screen(&mut self, screen: Box<dyn Screen>) {
        self.next_screen = Some(screen);
    }
}