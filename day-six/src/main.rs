use std::fs;
fn main() {
    let path = "day-six/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");

    let marker = get_marker(content);
    println!("The marker is \"{}\"", marker.unwrap());
}

fn get_marker(content: String) -> Option<usize> {
    let mut marker = "".to_string();
    for (index, character) in content.chars().enumerate() {
        if let Some(position_same_character) = marker.chars().position(|x| x == character) {
            marker.drain(..=position_same_character);
        }
        if marker.len() == 14 {
            return Some(index);
        }
        marker.push(character);
    }
    None
}
