use terminal_size::{Width, Height, terminal_size};

pub struct Dimension {
    width:  u16,
    height: u16
}

pub fn get_dimensions() -> Result<Dimension, String> {
    let term_size = terminal_size();
    let mut dim   = Dimension{width: 0, height: 0};

    if let Some((Width(x), Height(y))) = term_size {
        dim.width  = x;
        dim.height = y;
    }

    match dim.width  > 0 
       && dim.height > 0 {
        true  => Ok(dim),
        false => Err("Failed to get terminal size!".to_string())
    }
}

fn main() {
    let dim = match get_dimensions() {
        Ok(result) => result,
        Err(msg) => panic!("{}", msg)
    };

    println!("Terminal Size: {}x/{}y", dim.width, dim.height);

}
