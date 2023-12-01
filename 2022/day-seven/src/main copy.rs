use std::fs;

fn main() {
    let path = "day-five/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    get_directories(content);
}
#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    size: usize,
    directories: Option<Box<Directory>>,
}
#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}
impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

impl Directory {
    pub fn new(name: String, files: Vec<File>, directories: Option<Directory>) -> Self {
        let mut size = files.iter().map(|x| x.size).sum();
        // if directories.is_some() {
        //     size += directories
        //         .as_ref()
        //         .unwrap()
        //         .files
        //         .iter()
        //         .map(|x| x.size)
        //         .sum::<usize>();
        // }

        let directories = if let Some(directory) = directories {
            Some(Box::from(directory))
        } else {
            None
        };

        Self {
            name,
            files: files.clone(),
            size,
            directories,
        }
    }
}

fn get_directories(content: String) {
    let mut other_directory = Directory::new(
        "Foo".to_string(),
        vec![File::new("Bar".to_string(), 3)],
        None,
    );

    other_directory.directories = Some(Box::new(Directory::new(
        "Hello".to_string(),
        vec![File::new("World".to_string(), 122)],
        None,
    )));

    let mut directories: Directory = Directory::new(
        "Hello".to_string(),
        vec![File::new("World".to_string(), 124)],
        Some(other_directory),
    );

    println!("{:#?}", directories)
}
