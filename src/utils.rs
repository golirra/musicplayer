use iced::Task;
#[derive(Clone, Debug)]
pub enum UtilAction {
    ScanFileLibrary,
    FileLibraryScanned(Vec<String>),
}

#[derive(Debug, Default)]
pub struct Util {
    files: Vec<String>,
}

impl Util {
    pub fn new() -> Self {
        //Make a new vec to hold the filenames
        Self {
            files: Vec::new(),
        }
    }
    pub async fn scan_library() -> Vec<String> {
        let mut filenames = Vec::new();
        if let Ok(mut entries) = tokio::fs::read_dir("./").await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(name) = entry.file_name().into_string() {
                    filenames.push(name);
                }
            }
        }
        dbg!(&filenames);
        filenames
            
    }
}
/*
AudioAction::ScanFileLibrary => Task::perform(

                Self::scan_library(),
                AudioAction::FileLibraryScanned
            ),
            AudioAction::FileLibraryScanned => {
                Task::none()
            },
*/
/*
            }
*/
