mod screen;
use screen::{Render, Screen};

fn main() {
    let mut count = 0u32;
    let screen = Screen::new(30);
    screen.do_loop(|| {
        count += 1;
        println!("{}", count);
        count == 10
    })
}
