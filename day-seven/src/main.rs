use std::fs;

fn main() {
    let path = "day-seven/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let result = get_directories(content);
    println!("The result of part 1 is: {}.", result)
}
#[derive(Debug)]
struct Directory {
    name: String,
    size: usize,
    parent_directories: Vec<String>,
}
impl Directory {
    pub fn new(name: String, size: usize, parent_directories: Vec<String>) -> Self {
        Self {
            name,
            size,
            parent_directories,
        }
    }
}
#[derive(Debug)]
enum Message<'a> {
    CD(&'a str),
    LS(&'a str),
}

fn get_directories(content: String) -> usize {
    let mut file_path: Vec<String> = Vec::new();
    // let mut current_directory = "/".to_string();
    let mut directories: Vec<Directory> = vec![Directory::new("/".to_string(), 0, vec![])];

    for lines in content.split('$') {
        if lines.is_empty() {
            continue;
        }
        // println!("hi {}", lines);
        let lines = lines.trim();
        let command = if lines.starts_with("cd") {
            Message::CD(lines[2..].trim())
        } else {
            Message::LS(lines[2..].trim())
        };

        match command {
            Message::CD(dir) => {
                if dir == ".." {
                    // current_directory =
                    file_path.pop().unwrap();
                    // dbg!(&file_path);
                    continue;
                }
                // current_directory = dir.to_owned();
                file_path.push(dir.to_string());
                // dbg!(&command, &directories, &file_path);
            }
            Message::LS(files) => {
                // with files I also mean directories
                for file in files.lines() {
                    let file = file.trim();
                    if file.starts_with("dir") {
                        // directories
                        let name = file[4..].trim().to_string();

                        directories.push(Directory::new(name, 0, file_path.clone()));
                    } else {
                        // files

                        let end_size_index = file.find(" ").unwrap();
                        let size: usize = file[..end_size_index].trim().parse().unwrap();
                        let index = directories
                            .iter()
                            .position(|x| &x.name.as_str() == file_path.last().unwrap())
                            .unwrap();

                        directories.get_mut(index).unwrap().size += size;

                        let parent_dirs =
                            &directories.get(index).unwrap().parent_directories.clone();

                        parent_dirs.iter().for_each(|x| {
                            let index = directories.iter().position(|dir| &dir.name == x).unwrap();
                            directories.get_mut(index).unwrap().size += size;
                        });
                    }
                    // dbg!(&command, &directories, &file_path);
                }
            }
        }
    }

    dbg!(&directories);
    let sum: usize = directories
        .iter()
        .filter(|dir| dir.size <= 100000 && dir.size > 0)
        .map(|dir| {
            println!("{:?}", dir);
            dir.size
        })
        .sum();
    return sum;
}
