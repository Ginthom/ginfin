mod input_reader;

struct Entry {
    amount: f32,
    time: String,
}

pub fn create_entry() {
    let amount_new = input_reader::input_f32("Enter amount: ");
    let time_new = input_reader::input_string("Enter time: ");

    let entry = Entry {
        amount: amount_new,
        time: time_new
    };

    //Do something with entry i guess
}

pub fn view_entries() {}

pub fn handle_input() {
    let input: i32 = input_reader::input_i32("Input: ");
    println!("You used option {}", input);

    match input {
    1 => create_entry(),
    2 => view_entries(),
    3 => std::process::exit(0),
    -1 => (),
    _ => println!("Invalid input")
    };
}

pub fn display_menu() {
    println!("❱ Ginfin ❰\n");
    println!("Options:\n1) New Entry\n2) View Entries\n3) Exit\n");

}

fn main() {

    //print menu or something
    display_menu();
    handle_input();

    //Add 'entries'
    //fill up entries with values
    //write entries to file
    //load entries from file
    //display entries
    //calulate graph for
    //  week
    //  month
    //  year
    //display graph

}
