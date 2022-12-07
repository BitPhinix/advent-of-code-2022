use std::{collections::HashMap, fs};

#[derive(Debug)]
enum FSEntry {
    File(File),
    Directory(Directory),
}

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Directory {
    children: HashMap<String, usize>,
    parent: usize,
}

#[derive(Debug)]
struct AppendOnlyFS {
    current: usize,
    entries: Vec<FSEntry>,
}

impl AppendOnlyFS {
    fn move_to_parent(&mut self) -> Result<(), ()> {
        let current_dir = self.entries.get(self.current).ok_or(())?;
        let current_dir = match current_dir {
            FSEntry::Directory(dir) => dir,
            _ => return Err(()),
        };

        self.current = current_dir.parent;
        Ok(())
    }

    fn insert_file(&mut self, name: &str, size: u32) -> Result<(), ()> {
        let current_dir = self.entries.get(self.current).ok_or(())?;
        let current_dir = match current_dir {
            FSEntry::Directory(dir) => dir,
            _ => return Err(()),
        };

        if current_dir.children.contains_key(name) {
            return Err(());
        }

        let new_file = File { size };
        let new_file_index = self.entries.len();
        let current_dir = self.entries.get_mut(self.current).ok_or(())?;
        let current_dir = match current_dir {
            FSEntry::Directory(dir) => dir,
            _ => return Err(()),
        };

        current_dir.children.insert(name.into(), new_file_index);
        self.entries.push(FSEntry::File(new_file));
        Ok(())
    }

    fn move_into_or_insert(&mut self, name: &str) -> Result<(), ()> {
        let current_dir = self.entries.get(self.current).ok_or(())?;
        let current_dir = match current_dir {
            FSEntry::Directory(dir) => dir,
            _ => return Err(()),
        };

        if let Some(child_idx) = current_dir.children.get(name) {
            return match self.entries.get(*child_idx) {
                Some(FSEntry::Directory(_)) => {
                    self.current = *child_idx;
                    Ok(())
                }
                _ => Err(()),
            };
        }

        let new_dir = Directory {
            children: HashMap::new(),
            parent: self.current,
        };

        let new_dir_index = self.entries.len();
        let current_dir = self.entries.get_mut(self.current).ok_or(())?;
        let current_dir = match current_dir {
            FSEntry::Directory(dir) => dir,
            _ => return Err(()),
        };

        current_dir.children.insert(name.into(), new_dir_index);
        self.entries.push(FSEntry::Directory(new_dir));
        self.current = new_dir_index;
        Ok(())
    }

    fn size(&self, pos: &usize) -> Option<u32> {
        let mut size = 0;
        let mut to_visit: Vec<&usize> = vec![pos];
        while let Some(current) = to_visit.pop() {
            let current = self.entries.get(*current)?;
            match current {
                FSEntry::Directory(dir) => {
                    dir.children.iter().for_each(|(_, value)| {
                        to_visit.push(value);
                    });
                    continue;
                }
                FSEntry::File(file) => {
                    size += file.size;
                }
            };
        }

        return Some(size);
    }
}

fn main() -> Result<(), ()> {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut fs = AppendOnlyFS {
        current: 0,
        entries: vec![FSEntry::Directory(Directory {
            children: HashMap::new(),
            parent: 0,
        })],
    };

    for line in input.lines().skip(1) {
        if line == "$ cd .." {
            fs.move_to_parent()?;
            continue;
        }

        if line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let (_, name) = line.split_at(4);
            fs.move_into_or_insert(name)?;
            continue;
        }

        if line.starts_with("dir") {
            continue;
        }

        let (size, name) = line.split_once(' ').unwrap();
        fs.insert_file(name, size.parse().unwrap())?;
    }

    let dir_sizes: Vec<u32> = fs
        .entries
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &FSEntry)>>()
        .iter()
        .filter_map(|(i, entry)| match entry {
            FSEntry::Directory(_) => fs.size(i),
            _ => None,
        })
        .collect();

    let part_1: u32 = dir_sizes.iter().filter(|size| **size <= 100000).sum();
    println!("Part1: {}", part_1);

    let total_size = dir_sizes.first().unwrap();
    let to_free = 30000000 - (70000000 - total_size);
    let mut valid_dirs: Vec<&u32> = dir_sizes.iter().filter(|size| **size >= to_free).collect();
    valid_dirs.sort();
    println!("Part2: {}", valid_dirs.first().unwrap());

    return Ok(());
}
