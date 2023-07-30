pub trait Extention {
    fn has_extention(&self, ext: &str) -> bool;

    fn with_extention(&self, new_ext: &str) -> String;
}

impl Extention for str {
    fn has_extention(&self, ext: &str) -> bool {
        let to_find = ".".to_string() + ext;

        self.rfind(&to_find).is_some()
    }

    fn with_extention(&self, ext: &str) -> String {
        match (self.rfind('/'), self.rfind('.')) {
            (Some(start_name), Some(start_ext)) => {
                if start_ext > start_name {
                    self[..start_ext].to_owned() + "." + ext
                } else {
                    self.to_owned() + "." + ext
                }
            }
            (None, Some(start_ext)) => self[..start_ext].to_owned() + "." + ext,
            (_, None) => self.to_owned() + "." + ext,
        }
    }
}

#[test]
fn file_with_extention() {
    assert_eq!("test.txt", "test.err".with_extention("txt"));
}

#[test]
fn path_with_extention() {
    assert_eq!(
        "directory.something/test.txt",
        "directory.something/test.err".with_extention("txt")
    );
}

#[test]
fn file_without_extention() {
    assert_eq!("test.txt", "test".with_extention("txt"));
}

#[test]
fn path_without_extention() {
    assert_eq!(
        "directory.something/test.txt",
        "directory.something/test".with_extention("txt")
    );
}
