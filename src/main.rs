use std::fs;
use clap::Parser;
use serde_json::json;

#[derive(Parser)]
#[command(name = "save_editor", about = "Vampire Crawlers save editor")]
struct Args {
    /// Specify the game save using "C:\path\to\saveSaveProfile0.save". --
    /// If the parameter is empty, use the default path: %USERPROFILE%AppData\LocalLow\Nosebleed Interactive\Vampire Crawlers\Save\SaveProfile0.save
    #[arg(long)]
    path: Option<String>,

    #[arg(long)]
    coins: Option<i64>,

    /// Add one or more characters by short name (e.g Pugnala Giovanna)
    #[arg(long, num_args = 1..)]
    add_character: Vec<String>,
}

const CHARACTERS: &[&str] = &[
    "SimpleAchievement_Character_Antonio",
    "SimpleAchievement_Character_Pugnala",
    "SimpleAchievement_Character_Giovanna",
    "SimpleAchievement_Character_Concetta",
    "SimpleAchievement_Character_Poppea",
    "SimpleAchievement_Character_MissingNo",
    "MetricAchievement_Character_Gennaro",
    "MetricAchievement_Character_Arca",
    "MetricAchievement_Character_Poe",
    "MetricAchievement_Character_Dommario",
    "MetricAchievement_Character_Ramba",
    "MetricAchievement_Character_Porta",
    "MetricAchievement_Character_Mortuccio",
    "MetricAchievement_Character_Cavallo",
    "MetricAchievement_Character_Krochi",
    "MetricAchievement_Character_Clerici",
    "MetricAchievement_Character_OSole",
    "MetricAchievement_Character_Christine",
];
fn parse_save(file: &str) -> serde_json::Value {
    let read_file = fs::read_to_string(file).expect("Something went wrong reading the file");
    serde_json::from_str::<serde_json::Value>(&read_file).expect("Failed to parse JSON")
}

fn write_save(path: &str, data: &serde_json::Value) {
    let serialized = serde_json::to_string_pretty(data).unwrap();
    fs::write(path, serialized).expect("Failed to save file");
    println!("Saved to {}", path);
}

fn read_fields(data: &serde_json::Value) -> (String, i64, i64) {
    let profile_id = data["Data"]["ProfileId"]
        .as_str()
        .expect("ProfileId not found")
        .to_string();
    let coins_profile = data["Data"]["ProfileSaveData"]["TotalCoins"]
        .as_i64()
        .expect("ProfileSaveData.TotalCoins not found");
    let coins_progression = data["Data"]["ProgressionSaveData"]["TotalCoins"]
        .as_i64()
        .expect("ProgressionSaveData.TotalCoins not found");
    (profile_id, coins_profile, coins_progression)
}

fn default_save_path() -> String {
    let user = std::env::var("USERPROFILE")
        .expect("USERPROFILE not set");
    format!(r"{}\AppData\LocalLow\Nosebleed Interactive\Vampire Crawlers\Save\SaveProfile0.save", user)
}

fn print_fields(label: &str, fields: &(String, i64, i64)) {
    println!("\n-------------- [!] {} --------------", label);
    println!("Profile Name: {}", fields.0);
    println!("Total Coins on Profile: {}", fields.1);
    println!("Total Coins on Progression: {}", fields.2);
}

fn is_unlocked(data: &serde_json::Value, key: &str) -> bool {
    let achiviements = data["Data"]["ProgressionSaveData"]["AchievementsUnlocked"]
        .as_array()
        .expect("AchievementsUnlocked not found or not an array");

    achiviements.iter().any(|item| {
        item["Key"].as_str() == Some(key) && item["Value"].as_bool() == Some(true)
    })
}

fn add_achievement(data: &mut serde_json::Value, key: &str) {
    let achiviements = data
        .pointer_mut("/Data/ProgressionSaveData/AchievementsUnlocked")
        .unwrap()
        .as_array_mut()
        .unwrap();

    achiviements.push(json!({"Key": key, "Value": true}));
}

fn resolve_short_name(short: &str) -> Option<&'static str> {
    CHARACTERS.iter().find(|c| c.ends_with(&format!("_{}", short))).copied()
}

fn print_character_status(data: &serde_json::Value) {
    println!("\n-------------- [!] CHARACTERS --------------");
    for char_full in CHARACTERS.iter() {
        let short_name = char_full.rsplit_once('_').map(|(_, name)| name).unwrap_or(char_full);

        let mark = if is_unlocked(data, char_full) {'X'} else {' '};
        println!("[{}]  {}", mark, short_name);
    }

    let missing = CHARACTERS.iter().filter(|c| !is_unlocked(data, c)).count();
    println!("\nMissing: {} / {}", missing, CHARACTERS.len());
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(default_save_path);
    let mut data = parse_save(&path);

    let old_fields = read_fields(&data);
    print_fields("OLD VALUES", &old_fields);
    print_character_status(&data);

    let mut changed = false;

    // Coins
    if let Some(new_coins) = args.coins {
        *data.pointer_mut("/Data/ProfileSaveData/TotalCoins").unwrap()
            = json!(new_coins);
        *data.pointer_mut("/Data/ProgressionSaveData/TotalCoins").unwrap()
            = json!(new_coins);
        changed = true;
    }

    // Characters
    for short in &args.add_character {
        let full = match resolve_short_name(short) {
            Some(name) => name,
            None => {
                eprintln!("[!] Unknow character {}. Skipping.", short);
                continue;
            }
        };

        if is_unlocked(&mut data, full) {
            println!("[*] {} Already unlocked. Skipping.", short);
            continue;
        }

        add_achievement(&mut data, full);
        println!("[+] Added: {}", short);
        changed = true;
    }

    if changed {
        *data.pointer_mut("/Checksum").unwrap() = json!("");

        let new_fields = read_fields(&data);
        print_fields("NEW VALUES", &new_fields);
        print_character_status(&data);

        write_save(&path, &data);
    } else {
        println!("\n(No changes. Read-only mode.)");
    }

}