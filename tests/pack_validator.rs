use sokoworld::level::Level;

#[test]
// ensures all TOML level packs load from disk as valid Packs
fn test_packs_are_valid() {
    let paths = std::fs::read_dir("./assets/packs/").unwrap();

    for path in paths {
        let file_string =
            std::fs::read_to_string(path.unwrap().path()).expect("couldn't read file");
        let pack = sokoworld::level::pack::Pack {
            file: None,
            ..toml::from_str(file_string.as_str()).unwrap()
        };
        assert!(!pack.title.is_empty());

        for level in pack.levels {
            let level = Level::parse(&level).unwrap();
            assert!(level.is_valid(), "Level not valid: {:#?}", level);
        }
    }
}
