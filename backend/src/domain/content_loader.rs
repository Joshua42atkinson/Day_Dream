use std::collections::HashMap;
use common::{Quest, CharacterTemplate};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct GameData {
    pub quests: HashMap<String, Quest>,
    pub characters: HashMap<String, CharacterTemplate>,
}

pub fn load_game_data() -> Result<GameData, std::io::Error> {
    let quests = load_from_directory("content/quests")?;
    let characters = load_from_directory("content/characters")?;

    Ok(GameData { quests, characters })
}

fn load_from_directory<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<HashMap<String, T>, std::io::Error> {
    let mut data_map = HashMap::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path)?;
            let data: T = serde_json::from_str(&content)?;
            data_map.insert(file_stem, data);
        }
    }
    Ok(data_map)
}
