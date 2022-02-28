use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button,MessageDialog,Label,ButtonsType, DialogFlags, MessageType, Window,Popover,MenuButton};
use csv;
use std::fs::File;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

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
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Periodic table")
        .child(&grid)
        .build();
    let mut elements:Vec<Element> = vec![];

    let mut skipped_elements = 0;

    let mut message = MessageDialog::builder()
            .text("testing")
            .build();
        message.connect_response(|dialog, _| dialog.close());
    message.set_transient_for(Some(&window));
    
    let s = Rc::new(RefCell::new(message));

    // Make Element objects in list "elements" and attach them to the grid
    for i in 0..periodic_table.len(){
        if periodic_table[i][8] != "".to_string(){ // skip f block
            // add new element to elements array
            elements.push(Element::new( &periodic_table[i][1].clone(),
                                        &periodic_table[i][17].clone(),
                                        periodic_table[i][0].parse().unwrap(),
                                        periodic_table[i][2].clone(),
                                        &s,));

            // set window as transient for new element dialog
            elements[i-skipped_elements].dialog.set_transient_for(Some(&window));
            
            // attach new element to the grid
            grid.attach(&elements[i-skipped_elements].but,
                periodic_table[i][8].parse().unwrap(),
                periodic_table[i][7].parse().unwrap(),
                1,1);
        } else{
            skipped_elements+=1;
        }
    }


    

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
    name:String,
    electronegativity:String,
    atomic_number:i32,
    symbol:String,
    but:MenuButton,
    dialog:gtk4::MessageDialog,
    popover_text:gtk4::Label,
    popover:gtk4::Popover,    
}

// makes object type "MakeButton" with 2 functions that must be defined per-object
trait MakeButton{
    fn make_button(&mut self,ref_cell:&Rc<RefCell<MessageDialog>>);
    fn new(name:&str,electronegativity:&str,atomic_number:i32,symbol:String,ref_cell:&Rc<RefCell<MessageDialog>>) -> Element;
}

// makes Element object "Element" a "MakeButton" type
impl MakeButton for Element{
    fn make_button(&mut self,ref_cell:&Rc<RefCell<MessageDialog>>){
        let label = self.symbol.clone();
        let number = self.atomic_number.clone();
        /*let message = MessageDialog::builder()
            .text("testing")
            .build();
        message.connect_response(|dialog, _| dialog.close());*/
        
        let s2 = Rc::clone(ref_cell);
        
        //build popover label
        let popover_labal = Label::builder()
            .label(&[
                self.name.clone(),"\n".to_string(),
                self.symbol.clone(),"\n".to_string(),
                "Electronegativity: ".to_string(),self.electronegativity.clone(),"\n".to_string(),
                "#".to_string(),self.atomic_number.to_string()].join("").as_str())
            .build();
        self.popover_text = popover_labal;
        
        //build popover
        let popover = Popover::builder()
            .child(&self.popover_text)
            .autohide(true)
            .focusable(true)
            .has_arrow(true)
            .build();
        self.popover = popover;
        
        //build button for popover
        let but = MenuButton::builder()
            .label(&self.symbol)
            .hexpand(true)
            .vexpand(true)
            .popover(&self.popover)
            .build();
        
        
        self.but = but;
    }
    fn new(name:&str,electronegativity:&str,atomic_number:i32,symbol:String,ref_cell:&Rc<RefCell<MessageDialog>>) -> Element{
        let mut output:Element = Element{
            name:name.to_string(),
            electronegativity:electronegativity.to_string(),
            atomic_number:atomic_number,
            symbol:symbol,
            but:MenuButton::new(),
            dialog:MessageDialog::builder().build(),
            popover_text:gtk4::Label::builder().label("testing").build(),
            popover:Popover::builder().build(),
        };
        output.make_button(&ref_cell);
        return output;
    }
}