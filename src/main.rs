use rand::Rng;
use std::f64::consts::PI;
use std::io;

fn rainbow(freq: f64, i: f64) -> (u8, u8, u8) {
    let red = (freq * i + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    (red as u8, green as u8, blue as u8)
}

fn main() {
    let seed = rand::thread_rng().gen::<u32>();

    for line in io::stdin().lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            let color = rainbow(0.1, f64::from(seed + i as u32));
            print!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, c);
        }
        print!("\n");
    }

    print!("\x1b[0m");
}
