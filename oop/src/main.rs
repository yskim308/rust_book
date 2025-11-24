mod state;
mod traits;

fn main() {
    let mut screen = traits::Screen { components: vec![] };

    let button = Box::new(traits::Button {
        width: 75,
        height: 10,
        label: String::from("Hello from box"),
    });

    let select_box = Box::new(traits::SelectBox {
        width: 100,
        height: 30,
        options: vec![
            String::from("option1"),
            String::from("option2"),
            String::from("option1"),
        ],
    });

    screen.components.push(button);
    screen.components.push(select_box);

    screen.run();
}
