use rusqlite::{Connection, Result};
pub fn get_song_path(conn: &Connection, song_name: &str) -> Result<String> {
    let mut stmt = conn.prepare("SELECT path FROM files WHERE name = ? AND attribs = 32")?;
    let song_path: String = stmt.query_row([song_name], |row| row.get(0))?;
    Ok(song_path)
}

