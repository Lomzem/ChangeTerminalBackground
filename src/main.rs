use std::{fs, io::{Write, self, Read}};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut new_image = String::new();
    io::stdin().read_to_string(&mut new_image).unwrap();
    const JSON_SETTINGS_FILE: &str = "C:/Users/Lomzem/AppData/Local/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json";
    let contents = fs::read_to_string(JSON_SETTINGS_FILE).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    if let serde_json::Value::String(ref mut string) = &mut json["profiles"]["defaults"]["backgroundImage"] {
        *string = String::from(new_image);
    }
    let json_string = json.to_string();
    let mut file = fs::File::create(JSON_SETTINGS_FILE).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
