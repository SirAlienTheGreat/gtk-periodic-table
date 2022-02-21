use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use csv;
use std::fs::File;
use std::process;

fn main() {
    let application = Application::builder()
        .application_id("Tes.ting")
        .build();
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {

    let periodic_table = get_elements();

    let grid = gtk::Grid::new();
    let mut elements:Vec<Element> = vec![];

    let mut skipped_elements = 0;

    for i in 0..periodic_table.len(){
        if periodic_table[i][8] != "".to_string(){ // skip f block
            elements.push(Element::new(periodic_table[i][0].parse().unwrap(),periodic_table[i][2].clone()));
            grid.attach(&elements[i-skipped_elements].but,
                periodic_table[i][8].parse().unwrap(),periodic_table[i][7].parse().unwrap(),
                1,1);
        } else{
            skipped_elements+=1;
        }
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Periodic table")
        .child(&grid)
        .build();

    window.present();
}

fn get_elements() -> Vec<Vec<String>>{
    let mut output_record:Vec<csv::StringRecord> =vec![];
    // get list of StringRecord types
    // eg [StringRecord(["1", "Hydrogen", "H", "1.007", "0", . . . ]),]
    let file = File::open("PeriodicTable.csv").expect("Couldn't open file");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => output_record.push(record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }

    // convert list of StringRecords to list of list of strings
    let mut output: Vec<Vec<String>> = vec![];
    for i in 0..(output_record.len()){ // every row
        output.push(vec![]);
        for j in 0..(output_record[i].len()){ // every field
            output[i].push(output_record[i].get(j).expect("error").to_string())
        }
    }

    return output;
}


// makes variable properties of Element object
struct Element{
    atomic_number:i32,
    symbol:String,
    but:Button,
}

// makes object type "MakeButton" with 2 functions that must be defined per-object
trait MakeButton{
    fn make_button(&mut self);
    fn new(atomic_number:i32,symbol:String) -> Element;
}

// makes Element object "Element" a "MakeButton" type
impl MakeButton for Element{
    fn make_button(&mut self){
        let label = self.symbol.clone();
        let number = self.atomic_number.clone();
        let but = Button::builder()
            .label(&self.symbol)
            .hexpand(true)
            .vexpand(true)
            .build();
        but.connect_clicked(move |_|{
            println!("The atomic number of {} is {}",label,number);
        });
        self.but = but;
    }
    fn new(atomic_number:i32,symbol:String) -> Element{
        let mut output:Element = Element{
            atomic_number:atomic_number,
            symbol:symbol,
            but:Button::new(),
        };
        output.make_button();
        return output;
    }
}