use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label,Orientation};

fn main() {
    let application = Application::builder()
        .application_id("Tes.ting")
        .build();
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(app: &Application) {
    let mut horizontal_box = gtk::Box::new(Orientation::Horizontal,5);

    let mut group_1 = gtk::Box::new(Orientation::Vertical,5);
    horizontal_box.append(&group_1);

    let mut group_1_elements:Vec<Button> = vec![];
    let group_1_symbols = vec!["H","Li","Na","K","Rb","Cs","Fr"];
    let group_1_atomic_numbers = vec![1,3,11,19,37,55,87];
    for i in 0..7{
        group_1_elements.push(create_element(group_1_symbols[i],group_1_atomic_numbers[i]));
        group_1.append(&group_1_elements[i]);
    }

    let mut group_2 = gtk::Box::new(Orientation::Vertical,5);
    horizontal_box.append(&group_2);

    let mut group_3 = gtk::Box::new(Orientation::Vertical,5);
    horizontal_box.append(&group_3);

    let mut group_4 = gtk::Box::new(Orientation::Vertical,5);
    horizontal_box.append(&group_4);

    let button = Button::builder()
        .label("Does this show up?")
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Can you resize this?")
        .child(&horizontal_box)
        .build();

    window.present();
}

fn create_element(symbol:&str,atomic_number:i32) -> Button{
    let mut but = Button::builder()
        .label(&symbol)
        .build();
    but.connect_clicked(|_|{
                            println!("Element:, with atomic number:")
});
    return but;
}