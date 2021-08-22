struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    // Self with "S" can be replaced by Temperature
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0
        }
    }

    fn boiling() -> Self {
        Self {
            degrees_f: 200.0
        }
    }

    // it's borrowing
    fn show_temp(&self) {
        println!("{} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature {
        degrees_f: 99.9
    };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();

}