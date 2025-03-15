//Scan a file system and add all pieces to a sqlite database
use std::fs;
use std::path::Path;
use rusqlite::{params, Connection};
use anyhow::Result;

fn scan_directory(conn: &Connection, dir: &Path, parent_id: Option<i32>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let metadata = entry.metadata()?;

        // Determine file attributes (album folder, song file, album art)
        let attribs = if metadata.is_dir() {
            16 // Album folder
        } else if name.ends_with(".mp3") || name.ends_with(".flac") {
            32 // Song file
        } else if name.ends_with(".jpg") || name.ends_with(".png") {
            38 // Album art
        } else {
            continue; // Skip unknown files
        };

        // If it's a directory, insert the directory first and then recurse
        if metadata.is_dir() {
            // Insert the directory into the database first
            conn.execute(
                "INSERT INTO files (parentId, name, attribs, path) VALUES (?, ?, ?, ?)",
                params![parent_id, name, attribs, path.to_string_lossy()],
            )?;

            // Capture the last inserted row ID, which will be the parentId for files inside this directory
            let last_id = conn.last_insert_rowid() as i32;

            // Recurse to scan the contents of this directory
            scan_directory(conn, &path, Some(last_id))?;
        } else {
            // If it's a file, insert it with the parentId
            conn.execute(
                "INSERT INTO files (parentId, name, attribs, path) VALUES (?, ?, ?, ?)",
                params![parent_id, name, attribs, path.to_string_lossy()],
            )?;
        }
    }
    Ok(())
}

fn read_table(conn: &Connection){
    let x = conn.execute(
        "SELECT * FROM files LIMIT 1",
        params![]
    );
    dbg!(x);
}




fn main() -> Result<()> {
    let conn = Connection::open("music_library.db")?;
    let test_folder = Path::new("Music");

    // Start scanning from the root folder (parent_id is None for the root)
    scan_directory(&conn, test_folder, Some(0))?;

    Ok(())
}
