use chesto::{Element, ImageElement, InputEvent, Screen, TextElement};
use sdl2::pixels::Color;



#[derive(Clone)]
pub struct SecondScreen {
    pub elements: Vec<Box<dyn Element>>,
    pub next_screen: Option<Box<dyn Screen>>,
}

impl SecondScreen {
    pub fn new() -> Self {
        SecondScreen {
            elements: vec![
                Box::new(TextElement {text:"This is a second test.".to_string(), size: 1, color: None, rendered: false}),
                Box::new(TextElement {
                    text: "This is another second test.".to_string(), size: 1, color: None, rendered: false
                }),
                Box::new(ImageElement {
                    src: "/path/to/second.png".to_string(),
                    color: Some(Color::RGB(0, 255, 123)),
                    rendered: false,
                })
            ],
            next_screen: None,
        }
    }
}

impl Screen for SecondScreen {
    
    fn elements(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.elements
    }

    fn process(&mut self, input_events: InputEvent) -> bool {
        // own processing to override the base


        for element in self.elements() {
            element.process(input_events.clone());
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