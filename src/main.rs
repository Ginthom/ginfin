use terminal_size::{Width, Height, terminal_size};

fn main() {
    let size = terminal_size();

    if let Some((Width(w), Height(h))) = size {
        println!("Terminal Size: {}x/{}y", w, h);
    }

}
