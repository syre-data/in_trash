//! Determines whether a file is in the trash.
#[cfg(target_os = "windows")]
pub use windows::{in_trash, in_trash_file};

#[cfg(target_os = "windows")]
mod windows {
    use std::{
        fs,
        path::{Component, Path},
    };

    const TRASH_ROOT: &str = "$Recycle.Bin";

    pub fn in_trash(id: &file_id::FileId) -> Result<bool, file_path_from_id::Error> {
        let path = file_path_from_id::path_from_id(id)?;
        Ok(path_in_trash(&path))
    }

    pub fn in_trash_file(file: &fs::File) -> Result<bool, file_path_from_id::Error> {
        let path = file_path_from_id::path_from_file(file)?;
        Ok(path_in_trash(&path))
    }

    fn path_in_trash(path: impl AsRef<Path>) -> bool {
        let mut components = path.as_ref().components();
        let prefix = components.next().unwrap();
        assert!(matches!(prefix, Component::Prefix(_)), "invalid path");
        let root = components.next().unwrap();
        assert!(matches!(root, Component::RootDir), "invalid path");

        let Some(comp) = components.next() else {
            return false;
        };

        let Component::Normal(segment) = comp else {
            return false;
        };

        let Some(segment) = segment.to_str() else {
            return false;
        };

        segment == TRASH_ROOT
    }
}

#[cfg(test)]
mod test {
    use super::in_trash;
    use std::fs;
    use test_log::test;

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_not_in_trash() {
        const FILENAME: &str = "__tmp_out__";
        let path = std::env::current_dir().unwrap().join(FILENAME);
        let f = fs::File::create(&path).unwrap();
        let id = file_id::get_file_id(&path).unwrap();
        drop(f);
        assert!(!in_trash(&id).unwrap());
        fs::remove_file(&path).unwrap();
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_in_trash() {
        let f = tempfile::NamedTempFile::new().unwrap();
        let id = file_id::get_file_id(f.path()).unwrap();
        trash::delete(f.path()).unwrap();
        drop(f);
        assert!(in_trash(&id).unwrap());
    }
}
