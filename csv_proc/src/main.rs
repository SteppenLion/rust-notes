use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let data = "\
common name,length (cm)
Little book, 10
Big book, 20
Gigantic book, 50
Short book, 15
Invalid, data";
    let records = data.lines();

    for (i, records) in records.enumerate() {
        if i == 0 || records.trim().len() == 0 {
            continue;
        }

        let fields:Vec<_> = records.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprint!("debug: {:?} -> {:?}", records, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {} cm", name, length);
        }
    }
}
