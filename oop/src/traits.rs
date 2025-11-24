pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "drawing button with height: {}, width: {}, and label: {}",
            self.height, self.width, self.label
        );
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "drawing selectbox with height: {}, width: {}, and the following options:",
            self.height, self.width
        );
        for option in self.options.iter() {
            println!("{option},");
        }
    }
}
