use std::{io::Read, path::PathBuf, time::Instant};

use inquire::{Confirm, Text};
use log::{debug, error, info};
use patch_exe::{check_section_headers, patch_section_headers};

const GAME_APP_ID: u32 = 2199420; // Brickadia
const SERVER_APP_ID: u32 = 3017590; // Brickadia server
const GAME_BIN_NAME: &str = "BrickadiaSteam-Win64-Shipping.exe";
const SERVER_BIN_NAME: &str = "BrickadiaServer-Win64-Shipping.exe";

const FILE_UE4SS_SIG_CALL: &[u8] =
    include_bytes!("../../../assets/UE4SS_Signatures/CallFunctionByNameWithArguments.lua");
const FILE_UE4SS_SIG_CALL_NAME: &str = "UE4SS_Signatures/CallFunctionByNameWithArguments.lua";
const FILE_UE4SS_SIG_FNAME_CON: &[u8] =
    include_bytes!("../../../assets/UE4SS_Signatures/FName_Constructor.lua");
const FILE_UE4SS_SIG_FNAME_CON_NAME: &str = "UE4SS_Signatures/FName_Constructor.lua";
const FILE_UE4SS_SIG_FNAME_TO_STR: &[u8] =
    include_bytes!("../../../assets/UE4SS_Signatures/FName_ToString.lua");
const FILE_UE4SS_SIG_FNAME_TO_STR_NAME: &str = "UE4SS_Signatures/FName_ToString.lua";
const FILE_UE4SS_SIG_FTEXT_CON: &[u8] =
    include_bytes!("../../../assets/UE4SS_Signatures/FText_Constructor.lua");
const FILE_UE4SS_SIG_FTEXT_CON_NAME: &str = "UE4SS_Signatures/FText_Constructor.lua";
const FILE_UE4SS_SIG_PROC_LOCAL: &[u8] =
    include_bytes!("../../../assets/UE4SS_Signatures/ProcessLocalScriptFunction.lua");
const FILE_UE4SS_SIG_PROC_LOCAL_NAME: &str = "UE4SS_Signatures/ProcessLocalScriptFunction.lua";

const FILE_DWMAPI_DLL: &[u8] = include_bytes!("../../../assets/dwmapi.dll");
const FILE_DWMAPI_DLL_NAME: &str = "dwmapi.dll";
const FILE_UE4SS_DLL: &[u8] = include_bytes!("../../../assets/UE4SS.dll");
const FILE_UE4SS_DLL_NAME: &str = "UE4SS.dll";
const FILE_UE4SS_SETTINGS_INI: &[u8] = include_bytes!("../../../assets/UE4SS-settings.ini");
const FILE_UE4SS_SETTINGS_INI_NAME: &str = "UE4SS-settings.ini";
const FILE_VTABLE_LAYOUT_INI: &[u8] = include_bytes!("../../../assets/VTableLayout.ini");
const FILE_VTABLE_LAYOUT_INI_NAME: &str = "VTableLayout.ini";

fn copy_files_to_dir(dir: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let dirs_to_create = ["UE4SS_Signatures", "Mods"];

    for dir_name in dirs_to_create {
        let dir_path = dir.join(dir_name);
        if !dir_path.exists() {
            std::fs::create_dir(&dir_path).expect("Failed to create directory");
        }
    }

    let files_to_create = [
        (FILE_UE4SS_SIG_CALL, FILE_UE4SS_SIG_CALL_NAME),
        (FILE_UE4SS_SIG_FNAME_CON, FILE_UE4SS_SIG_FNAME_CON_NAME),
        (
            FILE_UE4SS_SIG_FNAME_TO_STR,
            FILE_UE4SS_SIG_FNAME_TO_STR_NAME,
        ),
        (FILE_UE4SS_SIG_FTEXT_CON, FILE_UE4SS_SIG_FTEXT_CON_NAME),
        (FILE_UE4SS_SIG_PROC_LOCAL, FILE_UE4SS_SIG_PROC_LOCAL_NAME),
        (FILE_DWMAPI_DLL, FILE_DWMAPI_DLL_NAME),
        (FILE_UE4SS_DLL, FILE_UE4SS_DLL_NAME),
        (FILE_UE4SS_SETTINGS_INI, FILE_UE4SS_SETTINGS_INI_NAME),
        (FILE_VTABLE_LAYOUT_INI, FILE_VTABLE_LAYOUT_INI_NAME),
    ];

    for (content, name) in files_to_create {
        let file_path = dir.join(name);
        if file_path.exists() {
            info!("File already exists, skipping: {name}");
            continue;
        }
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content).expect("Failed to write to file");
        info!("Created file: {}", name);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let startup = Instant::now();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    if !Confirm::new("I have read the NOTICE in the README and agree to the listed conditions.")
        .with_help_message("https://github.com/brickadia-community/br-lua-patcher")
        .with_default(false)
        .prompt()?
    {
        return Err("You must agree to the conditions to continue.".into());
    }

    let elapsed_ms = startup.elapsed().as_millis() as usize;
    let count_map = [("not", 8), ("or", 5), ("future", 2), ("a", 1), ("that", 3)];
    let (word, times) = count_map[elapsed_ms % count_map.len()];
    match Text::new(&format!(
        "You did, huh? How many times does the NOTICE have the word '{word}'?"
    ))
    .with_help_message("https://github.com/brickadia-community/br-lua-patcher")
    .prompt()
    {
        Ok(v) if v.parse().unwrap_or(0) == times => {}
        _ => return Err("Come back soon!".into()),
    };

    let steam_dir = steamlocate::SteamDir::locate()?;
    debug!("Steam directory: {}", steam_dir.path().display());

    if !steam_dir.path().exists() {
        error!("Missing steam directory.");
        return Err("Missing steam directory.".into());
    }

    let path_to_win64 = PathBuf::from("Brickadia\\Binaries\\Win64");

    'game: {
        let Some((game_dir, lib)) = steam_dir.find_app(GAME_APP_ID)? else {
            info!("Game directory not found.");
            break 'game;
        };

        let game_install = lib.resolve_app_dir(&game_dir);
        if !game_install.exists() {
            error!("Game directory does not exist: {}", game_install.display());
            break 'game;
        }
        debug!("Game directory: {}", game_install.display());
        let game_parent_dir = game_install.join(&path_to_win64);
        let game_bin = game_parent_dir.join(GAME_BIN_NAME);
        if !game_bin.exists() {
            error!("Game binary not found: {}", game_bin.display());
            break 'game;
        }

        if check_section_headers(&game_bin)? {
            info!("Game binary is already patched: {}", game_bin.display());
            break 'game;
        }

        if !Confirm::new("Patch and setup UE4SS for the game binary?")
            .with_default(true)
            .with_help_message(&format!("Game binary: {}", game_bin.display()))
            .prompt()?
        {
            info!("Skipping patching of game binary.");
            break 'game;
        }

        info!("Patching game binary: {}", game_bin.display());
        patch_section_headers(&game_bin)?;
        info!("Game binary patched! Copying files to game directory...");
        copy_files_to_dir(&game_parent_dir);
        info!("Files copied to game directory!");
    }

    'server: {
        let Some((server_dir, lib)) = steam_dir.find_app(SERVER_APP_ID)? else {
            info!("Server directory not found.");
            break 'server;
        };

        let server_install = lib.resolve_app_dir(&server_dir);
        if !server_install.exists() {
            error!(
                "Server directory does not exist: {}",
                server_install.display()
            );
            break 'server;
        }
        debug!("Server directory: {}", server_install.display(),);
        let server_parent_dir = server_install.join(&path_to_win64);
        let server_bin = server_parent_dir.join(SERVER_BIN_NAME);
        if !server_bin.exists() {
            error!("Server binary not found: {}", server_bin.display());
            break 'server;
        }

        if check_section_headers(&server_bin)? {
            info!("Server binary is already patched: {}", server_bin.display());
            break 'server;
        }

        if !Confirm::new("Patch and setup UE4SS for the server binary?")
            .with_default(true)
            .with_help_message(&format!("Server binary: {}", server_bin.display()))
            .prompt()?
        {
            info!("Skipping patching of server binary.");
            break 'server;
        }
        info!("Patching server binary: {}", server_bin.display());
        patch_section_headers(&server_bin)?;
        info!("Server binary patched! Copying files to server directory...");
        copy_files_to_dir(&server_parent_dir);
        info!("Files copied to server directory!");
    }
    info!("All Done! Press any key to close");
    let mut stdin = std::io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

    Ok(())
}
