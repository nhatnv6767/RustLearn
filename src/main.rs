struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn show_temp(&self) {
        println!("{} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature {
        degrees_f: 99.9
    };
    hot.show_temp();
}