pub mod archive_cracker;

pub struct ArchiveCracker {
    pub archive_path: String,
    pub password: String,
}

impl ArchiveCracker {
    pub fn new(archive_path: String, password: String) -> Self {
        ArchiveCracker { archive_path, password }
    }

    pub fn crack(&self) -> Result<(), String> {
        let file = File::open(&self.archive_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            if file.is_encrypted() {
                if self.try_password(&mut file) {
                    return Ok(());
                }
            }
        }
        Err("Password not found".to_string())
    }

    fn try_password(&self, file: &mut zip::read::ZipFile) -> bool {
        let password_regex = Regex::new(&self.password).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        password_regex.is_match(&String::from_utf8_lossy(&buffer))
    }
}