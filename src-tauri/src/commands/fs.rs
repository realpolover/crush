#[tauri::command]
pub async fn copy_file(from: String, to: String) -> Result<(), String> {
    use std::path::Path;

    if let Some(parent) = Path::new(&to).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }

    std::fs::copy(&from, &to)
        .map(|_| ())
        .map_err(|e| format!("Failed to copy file: {}", e))
}

#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<(), String> {
    use std::path::Path;
    use std::fs;

    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}
