struct Temperature {
    degrees_f: f64,
}

fn show_temp(temp: Temperature) {
    println!("{} degrees F", temp.degrees_f);
}

fn main() {
    let hot = Temperature {
        degrees_f: 99.9
    };
    show_temp(hot);
}