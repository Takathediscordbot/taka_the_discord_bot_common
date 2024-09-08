use super::models::Root;
use serde_json::from_str;
const REPLAY_DIRECTORY: &'static str = "./ttrm_replays";

#[test]
fn test_replay_parsing() {
    std::fs::read_dir(REPLAY_DIRECTORY).expect("Couldn't access replay directory")
    .flatten()
    .for_each(|f| {
        eprintln!("Reading file: {}", f.path().to_str().unwrap());
        let value = std::fs::read_to_string(format!("{}", f.path().to_str().unwrap())).unwrap();
        let _: Root = from_str(&value).unwrap();
        
    })
}

