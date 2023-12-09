// use std::fs;
use std::collections::HashMap;

type FileSystem = Vec<FileDirectory>;

pub struct FileSystemInterface {
    file_system: FileSystem,
    current_directory: Option<usize>
}

impl FileSystemInterface {
    fn new() -> FileSystemInterface {
        return FileSystemInterface{
            file_system: vec![FileDirectory::new(None)],
            current_directory: Some(0)
        }
    }

    pub fn parse(input:String) -> FileSystemInterface {
        let mut fsi = FileSystemInterface::new();
        let parser = FileDirectoryCommandParser::new();
        for line in input.split("\n") {
            if let Some(first_char) = line.chars().collect::<Vec<_>>().first() {
                if first_char == &'$' {
                    parser.run_command(&mut fsi, line);
                } else {
                    parser.add_file(&mut fsi, line);
                }
            }
        }
        return fsi;
    }

    // (dir_size, children_size)
    fn calculate_filesizes(&self, idx:usize) -> Vec<(usize, usize)> {
        let mut results:Vec<_> = self.file_system[idx].subdirectories.values()
            .flat_map(|child_idx|self.calculate_filesizes(*child_idx))
            .collect();
        let child_dir_sum:usize = results.clone().into_iter()
            .map(|(_dir_size, file_size)|file_size)
            .sum();
        let child_file_sum:usize = self.file_system[idx].files.values().sum();
        let dir_sum = child_dir_sum + child_file_sum;
        results.push((dir_sum, child_file_sum));
        return results;
    }

    fn switch_dir(&mut self, dir_name:&str) {
        self.current_directory = match dir_name {
            "/" => Some(0),
            ".." => self.file_system[self.current_directory.unwrap()].get_parent_idx(),
            _ => self.file_system[self.current_directory.unwrap()].get_subdirectory_idx(dir_name),
        };
    }

    fn create_dir(&mut self, dir_name:&str) {
        let new_idx = self.file_system.len();
        self.file_system.push(FileDirectory::new(self.current_directory));
        self.file_system[self.current_directory.unwrap()].add_directory(new_idx, dir_name);
    }

    fn create_file(&mut self, file_name:&str, filesize:usize) {
        self.file_system[self.current_directory.unwrap()].add_file(file_name, filesize);
    }
}

struct FileDirectory {
    parent_idx:Option<usize>,
    subdirectories:HashMap<String, usize>,
    files:HashMap<String, usize>,
}

impl FileDirectory {
    fn new(parent_idx: Option<usize>) -> FileDirectory {
        FileDirectory{
            parent_idx: parent_idx,
            subdirectories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_directory(&mut self, dir_idx:usize, dir_name:&str) {
        self.subdirectories.insert(dir_name.to_string(), dir_idx);
    }

    fn add_file(&mut self, file_name:&str, filesize:usize) {
        self.files.insert(file_name.to_string(), filesize);
    }

    fn get_parent_idx(&self) -> Option<usize> {
        self.parent_idx
    }

    fn get_subdirectory_idx(&self, name:&str) -> Option<usize> {
        self.subdirectories.get(name).cloned()
    }
}

struct FileDirectoryCommandParser { }

impl FileDirectoryCommandParser {
    fn new() -> FileDirectoryCommandParser {
        return FileDirectoryCommandParser{};
    }

    fn run_command(&self, fsi:&mut FileSystemInterface, input:&str) {
        let parts:Vec<_> = input.split(" ").collect();
        match parts[0] {
            "$"    => self.parse_user_input(fsi, input),
            _       => println!("Unhandled command: {}", input)
        }
    }

    fn add_file(&self, fsi:&mut FileSystemInterface, input:&str) {
        let parts:Vec<_> = input.split(" ").collect();
        if parts[0] == "dir" {
            fsi.create_dir(parts[1]);
        } else {
            fsi.create_file(parts[1], parts[0].parse::<usize>().unwrap());
        }
    }

    fn parse_user_input(&self, fsi:&mut FileSystemInterface, input:&str) {
        let parts:Vec<_> = input.split(" ").collect();
        match parts[1] {
            "cd"    => fsi.switch_dir(parts[2]),
            "ls"    => (),
            _       => todo!()
        }
    }
}

pub fn lowest_to_free(fsi:&FileSystemInterface, available_space:usize, required_free_space:usize) -> usize {
    let file_sizes = fsi.calculate_filesizes(0).into_iter()
        .map(|(dir_size, _file_size)|dir_size);

    if let Some(total_filesize) = file_sizes.clone().max() {
        let free_space = available_space - total_filesize;
        if free_space >= required_free_space {
            return 0;
        }
        let minimum_to_delete = required_free_space - free_space;

        return file_sizes
            .filter(|size|size >= &minimum_to_delete)
            .min().unwrap();

    }

    panic!("Something happened");
}

pub fn sum_filesizes_under_100000(fsi:&FileSystemInterface) -> usize {
    return fsi.calculate_filesizes(0).into_iter()
    .filter(|(dir_size, _file_size)|*dir_size <= 100000)
    .map(|(dir_size, _file_size)|dir_size)
    .sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_test_input() -> FileSystemInterface {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        return FileSystemInterface::parse(input.to_string());
    }

    fn create_deep_fsi() -> FileSystemInterface {
        let mut fsi = FileSystemInterface::new();
        fsi.create_dir("aa");
        fsi.switch_dir("aa");
        fsi.create_dir("a");
        fsi.switch_dir("a");
        fsi.create_file("ss", 67);
        fsi.create_file("s2", 89);
        fsi.switch_dir("..");
        fsi.create_dir("f");
        fsi.switch_dir("f");
        fsi.create_file("ss", 12);
        fsi.create_file("s2", 2);
        return fsi;
    }

    #[test]
    fn test_parse_filesystem() {
        let fsi = get_test_input();
        assert_eq!(95437, sum_filesizes_under_100000(&fsi));
    }

    #[test]
    fn test_parse_deep_filesystem() {
        /*
        -aa (170)
         - a (170)
           - e (156)
             - 67
             - 89
           - f (14)
             - 12
             - 2


             <>
             14, 14
             <>
             156, 156
             a
             <(14, 14), (156, 156)>
             child_size = 14 + 156 = 170
             dir_size = 170 + 0
             170
             aa
             <170, 170>

             [14]
            [156]
            [14, 156, 170]
            [14, 156, 170, 340]
            14
            156
            170
            340
        */
        let fsi = create_deep_fsi();
        assert_eq!(510, sum_filesizes_under_100000(&fsi));
    }

    #[test]
    fn test_lowest_to_free() {
        let fsi = get_test_input();
        assert_eq!(24933642, lowest_to_free(&fsi, 70000000, 30000000));
    }



    #[test]
    fn test_delete_from_deep_filesystem() {
        /*
        -aa (170)
         - a (170)
           - e (156)
             - 67
             - 89
           - f (14)
             - 12
             - 2


             <>
             14, 14
             <>
             156, 156
             a
             <(14, 14), (156, 156)>
             child_size = 14 + 156 = 170
             dir_size = 170 + 0
             170
             aa
             <170, 170>

             [14]
            [156]
            [14, 156, 170]
            [14, 156, 170, 340]
            14
            156
            170
            340
        */
        let fsi = create_deep_fsi();
        assert_eq!(0, lowest_to_free(&fsi, 460, 200));
        assert_eq!(14, lowest_to_free(&fsi, 360, 200));
    }
}