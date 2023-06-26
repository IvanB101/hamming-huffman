impl Extention for &str {
    fn has_extention(&self, ext: &str) -> bool {
        match self.split('.').last() {
            Some(e) => e == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let parts = self.split('/');
        let mut path = "".to_owned();

        if parts.clone().count() > 1 {
            for part in parts.clone().take(parts.clone().count() - 1) {
                path += part;
                path += "/";
            }
        }

        let file = parts.last().unwrap_or(self);

        let name = match file.chars().rev().position(|x| x == '.') {
            Some(index) => file.split_at(file.len() - 1 - index).0,
            None => file,
        };

        path + name + "." + ext
    }
}

impl Extention for String {
    fn has_extention(&self, ext: &str) -> bool {
        match self.split('.').last() {
            Some(e) => e == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let parts = self.split('/');
        let mut path = "".to_owned();

        if parts.clone().count() > 1 {
            for part in parts.clone().take(parts.clone().count() - 1) {
                path += part;
                path += "/";
            }
        }

        let file = parts.last().unwrap_or(self);

        let name = match file.chars().rev().position(|x| x == '.') {
            Some(index) => file.split_at(file.len() - 1 - index).0,
            None => file,
        };

        path + name + "." + ext
    }
}

pub trait Extention {
    fn has_extention(&self, ext: &str) -> bool;

    fn with_extention(&self, new_ext: &str) -> String;
}
