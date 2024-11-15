#[cfg(not(target_family = "wasm"))]
use std::env::current_exe;
use std::path::PathBuf;

pub const ASSETS_DIR: &str = "assets";

/// Returns the PathBuf to the assets folder root
///
/// "intelligently" determines which path the assets dir is located at based on a few different
/// aspects, in this order:
/// 0. was the `--assets` arg provided?
/// 1. was the SOKOWORLD_ASSETS ENV set?
/// 2. are we running from Cargo where the assets are in the CARGO_MANIFEST_DIR?
/// 3. are we running from a build where assets are next to the binary?
/// 4. are we in a MacOS .app bundle where the assets are in `../Resources/assets`?
///
/// For Windows and Linux releases, it'll be #2. For macOS releases it'll be #3.
///
/// WASM does not do anything special.
///
/// Panics if it cannot determine a valid assets dir
#[cfg(not(target_family = "wasm"))]
pub fn determine_asset_path() -> PathBuf {
    // try to find assets from the `--assets` CLI arg
    let parsed_args = parse_args(env::args().collect());
    if let Some(arg) = parsed_args.iter().find(|arg| arg.key == "assets") {
        let mut cargo_path = PathBuf::new();
        cargo_path.push(&arg.value);
        return cargo_path;
    }

    // try to find assets dir in the SOKOWORLD_ASSETS dir
    use std::env;
    match std::env::var("SOKOWORLD_ASSETS") {
        Ok(sokoworld_assets_dir) => {
            let mut cargo_path = PathBuf::new();
            cargo_path.push(sokoworld_assets_dir);
            return cargo_path;
        }
        Err(_e) => (), // proceed to attempt next way
    }

    // try to find assets dir in the asset CLI dir
    match std::env::var("SOKOWORLD_ASSETS") {
        Ok(sokoworld_assets_dir) => {
            let mut cargo_path = PathBuf::new();
            cargo_path.push(sokoworld_assets_dir);
            return cargo_path;
        }
        Err(_e) => (), // proceed to attempt next way
    }

    // try to find assets dir in cargo project root
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(cargo_manifest_dir) => {
            let mut cargo_path = PathBuf::new();
            cargo_path.push(cargo_manifest_dir);
            cargo_path.push(ASSETS_DIR);
            return cargo_path;
        }
        Err(_e) => (), // proceed to attempt next way
    }

    match current_exe() {
        Ok(exe_path) => {
            // try to find assets dir located next to the executable
            let mut sibling_path = exe_path.clone();
            sibling_path.pop();
            sibling_path.push(ASSETS_DIR);
            if sibling_path.is_dir() {
                return sibling_path;
            } else {
                // try to find assets dir in MacOS bundle (Some.app/Contents/Resources/assets)
                let mut macos_app_path = exe_path.clone();
                macos_app_path.pop();
                macos_app_path.pop();
                macos_app_path.push("Resources");
                macos_app_path.push(ASSETS_DIR);
                if macos_app_path.is_dir() {
                    return macos_app_path;
                }
            }
        }
        Err(_e) => (),
    }
    panic!("asset path cannot be determined")
}

#[cfg(target_family = "wasm")]
pub fn determine_asset_path() -> PathBuf {
    PathBuf::from(r"assets")
}

#[cfg(not(target_family = "wasm"))]
#[derive(PartialEq, Debug)]
struct Arg {
    key: String,
    value: String,
}

#[cfg(not(target_family = "wasm"))]
/// Simple CLI arg parser for specifying values with `--foo bar` or `--foo=bar`; does not support
/// Boolean values
fn parse_args(args: Vec<String>) -> Vec<Arg> {
    let mut parsed_args: Vec<Arg> = vec![];
    for (i, arg) in args.iter().enumerate() {
        if arg.contains("=") {
            if arg.starts_with("-") {
                let parts: Vec<&str> = arg.split("=").collect();
                let key = parts[0].to_string().replace("--", "");
                parsed_args.push(Arg {
                    key: key.to_owned(),
                    value: parts[1].to_string(),
                });
            }
        } else if arg.starts_with("-") {
            let key = arg.replace("--", "");
            parsed_args.push(Arg {
                key: key.to_owned(),
                value: args[i + 1].to_string(),
            });
        }
    }
    parsed_args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_equal_args() {
        let args = parse_args(vec![String::from("--foo=bar")]);
        assert_eq!(
            args,
            vec![Arg {
                key: String::from("foo"),
                value: String::from("bar")
            }]
        );
    }

    #[test]
    fn it_parses_space_args() {
        let args = parse_args(vec![String::from("--foo"), String::from("bar")]);
        assert_eq!(
            args,
            vec![Arg {
                key: String::from("foo"),
                value: String::from("bar")
            }]
        );
    }

    #[test]
    fn it_parses_mixed_args() {
        let args = parse_args(vec![
            String::from("--foo=bar"),
            String::from("--name"),
            String::from("brett"),
        ]);

        assert_eq!(
            args,
            vec![
                Arg {
                    key: String::from("foo"),
                    value: String::from("bar")
                },
                Arg {
                    key: String::from("name"),
                    value: String::from("brett")
                }
            ]
        )
    }
}
