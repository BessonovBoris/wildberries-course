use std::collections::HashMap;

fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5, -35.0, 20.0];

    let intervals = group_temperatures_by_interval(temperatures);

    for (interval, values) in intervals {
        println!("{}: {:?}", interval, values);
    }
}

fn group_temperatures_by_interval(temperatures: Vec<f64>) -> HashMap<String, Vec<f64>> {
    let mut intervals: HashMap<String, Vec<f64>> = HashMap::new();

    for temperature in temperatures {
        let interval = get_interval(temperature);
        intervals
            .entry(interval)
            .or_insert_with(Vec::new)
            .push(temperature);
    }

    intervals
}

fn get_interval(temperature: f64) -> String {
    let interval_start = (temperature / 10.0).floor() * 10.0;
    let interval_end = interval_start + 10.0;
    format!("[{}, {})", interval_start, interval_end)
}