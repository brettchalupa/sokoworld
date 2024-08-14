#[test]
// ensures all TOML level packs load from disk as valid Packs
fn test_packs_are_valid() {
    let paths = std::fs::read_dir("./assets/packs/").unwrap();

    for path in paths {
        let file_string =
            std::fs::read_to_string(path.unwrap().path()).expect("couldn't read file");
        sokoworld::level::pack::Pack {
            file: None,
            ..toml::from_str(file_string.as_str()).unwrap()
        };
    }
}
