// Code taken from https://shadowmint.gitbooks.io/rust/content/howto/walk_directory.html

use log::debug;
use std::fs::read_dir;
use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;

pub struct Walk {
    path: PathBuf,
    targets: Vec<PathBuf>,
    reader: Option<ReadDir>,
    current: Option<PathBuf>,
}

impl Walk {
    /// Create a new walker
    pub fn new(path: &Path) -> Walk {
        let mut root = PathBuf::new();
        root.push(path);
        let mut rtn = Walk {
            path: root,
            targets: Vec::new(),
            reader: None,
            current: None,
        };
        rtn.reset();
        return rtn;
    }

    /// Reset this iterator
    pub fn reset(&mut self) {
        self.targets.clear();
        self.targets.push(self.path.clone());
    }

    /// Generate a new reader, if we currently have none
    /// @return false If no reader could be generated.
    fn next_reader(&mut self) -> bool {
        let mut rtn = true;
        let mut check = true;
        while check {
            if self.reader.is_none() {
                if let Some(target) = self.targets.pop() {
                    if let Ok(reader) = read_dir(target.clone()) {
                        self.reader = Some(reader);
                        self.current = Some(target);
                    } else {
                        // TODO: Collect errors
                        debug!("Failed to read dir: {:?}", target);
                        self.reader = None;
                    }
                } else {
                    check = false;
                    rtn = false;
                }
            } else {
                check = false;
            }
        }
        return rtn;
    }

    /// Return the next path, push it onto the set of targets if it's a directory
    pub fn next(&mut self) -> Option<PathBuf> {
        if self.next_reader() {
            if let Some(record) = self.reader.as_mut().unwrap().next() {
                if let Ok(path) = record {
                    // Generate filename
                    let mut rtn = self.path.clone();
                    rtn.push(self.current.as_ref().unwrap().clone());
                    rtn.push(path.file_name());

                    // Add this to the list of paths to open
                    if let Ok(tt) = path.file_type() {
                        if tt.is_dir() {
                            self.targets.push(rtn.clone());
                        }
                    }

                    return Some(rtn);
                }
                // I/O Error
                else {
                    debug!("IO ERror");
                    self.reader = None;
                    return self.next();
                }
            }
            // No records left on this iterator
            else {
                self.reader = None;
                return self.next();
            }
        }
        return None;
    }
}

#[cfg(test)]
mod test {

    use super::Walk;
    use log::debug;
    use std::path::Path;

    #[test]
    fn test_walk_dir() {
        let target = Path::new("/tmp");
        let mut walk = Walk::new(target);
        while let Some(p) = walk.next() {
            debug!("{:?}", p);
        }
    }
}
