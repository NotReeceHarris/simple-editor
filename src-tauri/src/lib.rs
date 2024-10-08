// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/* #[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} */

use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        //.invoke_handler(tauri::generate_handler![get_file_tree])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory("C:/Users/reece/OneDrive/Documents/Projects/simple-editor", false);
            dbg!(scope.allowed());
  
            Ok(())
         })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
