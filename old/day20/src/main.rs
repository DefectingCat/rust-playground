use day20::{button::*, screen::*, select_box::*};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: "Submit".to_string(),
            }),
            Box::new(SelectBox {
                width: 10,
                height: 10,
                options: vec!["hello".to_string(), "world".to_string()],
            }),
        ],
    };

    screen.run();
}
