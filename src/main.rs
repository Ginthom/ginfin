mod input_reader;

#[derive(Clone)]
struct Entry {
    amount: f32,
    time: String,
}

struct History {
    entries: Vec<Entry>,
}

fn create_entry(mut history: History) {
    let amount_new = input_reader::input_f32("Enter amount: ");
    let time_new = input_reader::input_string("Enter time: ");

    let entry = Entry {
        amount: amount_new,
        time: time_new
    };

    history.entries.push(entry);
}

fn view_entries(history: History) {
    let tmp_history: Vec<Entry> = history.entries.clone();

    println!("Your history:");

    for entry in tmp_history {
        println!("->{}: {}€", entry.time, entry.amount);
    }
}

fn handle_input(history: History) {
    let input: i32 = input_reader::input_i32("Input: ");
    println!("You used option {}", input);

    match input {
    1 => create_entry(history),
    2 => view_entries(history),
    3 => std::process::exit(0),
    -1 => (),
    _ => println!("Invalid input")
    };
}

fn display_menu() {
    println!("❱ Ginfin ❰\n");
    println!("Options:\n1) New Entry\n2) View Entries\n3) Exit\n");

}

fn main() {

    //Create history
    let history = History {
        entries:  Vec::new()
    };

    //Load history from file
    
    display_menu();
    handle_input(history);
    handle_input(history);
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
