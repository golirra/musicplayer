//Scan a file system and add their metadata to a sqlite database
use std::fs;
use rusqlite::{ToSql, types::ToSqlOutput};
use serde::{Serialize, Deserialize};
use std::path::Path;
use rusqlite::{params, Connection, Row};
use anyhow::Result;
use rusqlite::types::Type;
use rodio::Decoder;
use id3::{Tag as Tagg, Error as TE, TagLike, partial_tag_ok, no_tag_ok};

const MUSIC_FOLDER: &str = "C:/Users/webbs/programming/cs/rust/Rust-playground/src/Music";

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    title: String,
    album: String,
    artist: String,
    genre: String,
    year: String,
}

impl ToSql for Metadata {
    fn to_sql(&self) -> Result<ToSqlOutput, rusqlite::Error> {
        serde_json::to_string(self)
            .map(ToSqlOutput::from)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    }
}

pub fn scan_directory(conn: &Connection, dir: &Path, parent_id: Option<i32>) -> Result<()> {
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
        } else if attribs == 32 { //32 == Song file
            let tg = Tagg::read_from_path(&path)?;
            let md = Metadata {
                title: tg.title().unwrap().to_string(),
                album: tg.album().unwrap().to_string(),
                artist: tg.artist().unwrap().to_string(),
                genre: tg.genre().unwrap().to_string(),
                year: tg.year().unwrap().to_string(),
            };
                conn.execute(
                "INSERT INTO files (parentId, name, attribs, path, md)
                VALUES (?, ?, ?, ?, ?)",
                params![parent_id, name, attribs, path.to_string_lossy(), md],
            )?;

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

#[derive(Debug)]
pub struct File {
    id: usize,
    parentId: usize,
    name: String,
    attribs: usize,
    //path: String,
}

impl File {
    fn deserialize(row: &Row) -> Result<File, rusqlite::Error>{
        Ok(
            File {
                id: row.get(0)?,
                parentId: row.get(1)?,
                name: row.get(2)?,
                attribs: row.get(3)?,
                //path: row.get(4)?,
            }
        )
    }
}

pub fn read_table() -> Result<Vec<String>> {
    let db_path = "C:/Users/webbs/programming/cs/rust/musicplayer/src/music_library.db";
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT path FROM files WHERE attribs == 32")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut paths = Vec::new();
    for row_result in rows {
        paths.push(row_result?);
    }
    dbg!(&paths);

    //let fi = x.query_map([],|row| row.get(0) )?;
    //dbg!(fi.collect::<Vec<_>>().pop()); 
    Ok(paths)
}

pub fn setup_database() -> Result<()> {
    let db_path = "music_library.db";
    let music_path = Path::new(MUSIC_FOLDER);
    //If db exists destroy it
    if fs::metadata(db_path).is_ok() {
        fs::remove_file(db_path)?;
    }
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parentId INTEGER,
            name TEXT NOT NULL,
            attribs INTEGER,
            path TEXT NOT NULL,
            md TEXT
        )",
        [],
    )?;
    scan_directory(&conn, music_path, Some(0))?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("music_library.db")?;
    let test_folder = Path::new("Music");

    // Start scanning from the root folder (parent_id is None for the root)
    scan_directory(&conn, test_folder, Some(0))?;

    Ok(())
}
