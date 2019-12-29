const SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(number: u32) -> String {
    let mut result: String = String::new();
    let is_factor = |factor| number % factor == 0;

    for (factor, sound) in SOUNDS.iter() {
        if is_factor(*factor) {
            result.push_str(sound);
        }
    }

    if result.is_empty() {
        number.to_string()
    } else {
        result
    }
}
