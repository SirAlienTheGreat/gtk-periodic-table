mod periodic_table_array;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label,Popover,MenuButton,Justification,ArrowType};

fn main() {
    

    let application = Application::builder()
        .application_id("Tes.ting")
        .build();
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    let periodic_table = periodic_table_array::get_periodic_table();

    let grid = gtk::Grid::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Periodic table")
        .child(&grid)
        .build();
    let mut elements:Vec<Element> = vec![];

    let mut skipped_elements = 0;



    // Make Element objects in list "elements" and attach them to the grid
    for i in 0..periodic_table.len(){
        // skip f block
            // add new element to elements array
            elements.push(Element::new( &periodic_table[i][1].clone(),
                                        &periodic_table[i][17].clone(),
                                        periodic_table[i][3].parse().unwrap(),
                                        periodic_table[i][0].parse().unwrap(),
                                        periodic_table[i][2].clone(),
                                        ));

        if periodic_table[i][8] != "".to_string(){ // attach by coordinates if not in f block
            // attach new element to the grid
            grid.attach(&elements[i].but,
                periodic_table[i][8].parse().unwrap(),
                periodic_table[i][7].parse().unwrap(),
                1,1);
        } else{
            let mut row = 20;
            if skipped_elements >=14{
                row = 21;
            }
            grid.attach(&elements[i].but,(skipped_elements%14)+3, row-1,1,1);
            skipped_elements+=1;
        }
    }


    

    window.present();
}

/*fn get_elements() -> Vec<Vec<String>>{
    let mut output_record:Vec<csv::StringRecord> =vec![];
    // get list of StringRecord types
    // eg [StringRecord(["1", "Hydrogen", "H", "1.007", "0", . . . ]),]
    const PROJECT_DIR: Dir = include_dir!("resources");

    let file = File::open("PeriodicTable.csv").expect("Couldn't open file");
    //let file = include_bytes!("/PeriodicTable.csv").unwrap();
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
    println!("{:?}",output);
    return output;
}*/


// makes variable properties of Element object
struct Element{
    name:String,
    electronegativity:String,
    atomic_number:i32,
    mass:f32,
    symbol:String,
    but:MenuButton,
    popover_text:gtk4::Label,
    popover:gtk4::Popover,    
}

// makes object type "MakeButton" with 2 functions that must be defined per-object
trait MakeButton{
    fn make_button(&mut self);
    fn new(name:&str,electronegativity:&str,mass:f32,atomic_number:i32,symbol:String) -> Element;
}

// makes Element object "Element" a "MakeButton" type
impl MakeButton for Element{
    fn make_button(&mut self){

        
        //build popover label
        let popover_labal = Label::builder()
            .label(&[
                self.name.clone()," (".to_string(),
                self.symbol.clone(),")\n".to_string(),
                "#".to_string(),self.atomic_number.to_string(),"\n".to_string(),
                "Mass: ".to_string(),self.mass.to_string()," amu\n".to_string(),
                "Electronegativity: ".to_string(),self.electronegativity.clone(),"\n".to_string(),
                ].join("").as_str())
            .justify(Justification::Center)
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
            .direction(ArrowType::None)
            .popover(&self.popover)
            .build();
        
        self.but = but;
    }
    fn new(name:&str,electronegativity:&str,mass:f32,atomic_number:i32,symbol:String) -> Element{
        let mut electronegativity_with_na = electronegativity;
        if electronegativity_with_na == ""{
            electronegativity_with_na = "N/A";
        }

        let mut output:Element = Element{
            name:name.to_string(),
            electronegativity:electronegativity_with_na.to_string(),
            mass,
            atomic_number,
            symbol,
            but:MenuButton::new(),
            popover_text:gtk4::Label::builder().label("testing").build(),
            popover:Popover::builder().build(),
        };
        output.make_button();
        return output;
    }
}