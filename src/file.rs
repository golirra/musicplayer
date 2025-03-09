use std::fs;

pub fn get_filenames_in_directory(directory: &str) -> Vec<String> {
    fs::read_dir(directory)
        .unwrap()
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect()
}
