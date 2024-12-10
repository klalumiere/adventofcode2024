use std::fs;

const FREE_SPACE_TOKEN: isize = -1;

#[derive(Clone, Debug, PartialEq, Eq)]
struct File {
    position: isize,
    file_id: isize,
    size: isize
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FreeSpace {
    position: isize,
    size: isize
}

#[derive(Clone, Debug)]
struct Disk {
    blocks: Vec<isize>,
    files: Vec<File>,
    free_spaces: Vec<FreeSpace>,
}

impl Disk {
    fn from(disk_map: &str) -> Disk {
        const RADIX: u32 = 10;
        let disk_map = disk_map.chars()
            .map(|c| c.to_digit(RADIX).expect("Cant parse number"));
        let mut blocks: Vec<isize> = Vec::new();
        let mut file_id: isize = 0;
        for (i, n) in disk_map.enumerate() {
            let is_free_space = i % 2 != 0;
            for _ in 0..n {
                if is_free_space {
                    blocks.push(FREE_SPACE_TOKEN);
                } else {
                    blocks.push(file_id);
                }
            }
            if ! is_free_space {
                file_id += 1;
            }
        }
        let mut files: Vec<File> = Vec::new();
        let mut free_spaces: Vec<FreeSpace> = Vec::new();
        let mut i: usize = 0;
        while i < blocks.len() {
            let beginning = isize::try_from(i).expect("To fit into isize");
            let id = blocks[i];
            let mut size = 0;
            while i < blocks.len() && blocks[i] == id {
                size += 1;
                i += 1;
            }
            if id == FREE_SPACE_TOKEN {
                let free_space = FreeSpace { position: beginning, size };
                free_spaces.push(free_space);
            } else {
                let file = File { position: beginning, file_id: id, size };
                files.push(file);
            }
        }        
        Disk { blocks, files, free_spaces }
    }

    fn has_free_space(self: &Disk) -> bool {
        self.blocks.iter().any(|&block| block == FREE_SPACE_TOKEN)
    }

    fn calculate_checksum(self: &Disk) -> isize {
        self.blocks.iter().enumerate()
            .filter(|(_, &block)| block != FREE_SPACE_TOKEN)
            .map(|(i, block)| isize::try_from(i).expect("To fit into isize")*block).sum()
    }

    fn mv(self: & mut Disk, file: &File, free_space: &FreeSpace) {
        assert!(file.size <= free_space.size);
        for i in free_space.position..(free_space.position + file.size) {
            self.blocks[i as usize] = file.file_id;
        }
        for i in file.position..(file.position + file.size) {
            self.blocks[i as usize] = FREE_SPACE_TOKEN;
        }

        let file_index = self.files.iter()
            .position(|f| f == file)
            .expect("Free space not found");
        self.files[file_index] = File { position: free_space.position, ..file.clone() };

        let free_space_index = self.free_spaces.iter()
            .position(|fs| fs == free_space)
            .expect("Free space not found");
        self.free_spaces[free_space_index] = FreeSpace { size: free_space.size - file.size, position: free_space.position + file.size };
    }
}

fn compress_free_space(disk: &Disk) -> Disk {
    let mut blocks: Vec<isize> = Vec::with_capacity(disk.blocks.len());
    let mut reverse_it = disk.blocks.len() - 1;
    for (i, block) in disk.blocks.iter().enumerate() {
        if i >= reverse_it {
            if i == reverse_it && *block != FREE_SPACE_TOKEN {
                blocks.push(*block);
            }
            break;
        }
        if *block == FREE_SPACE_TOKEN {
            while disk.blocks[reverse_it] == FREE_SPACE_TOKEN && reverse_it > i {
                reverse_it -= 1;
            }
            blocks.push(disk.blocks[reverse_it]);
            reverse_it -= 1;
        } else {
            blocks.push(*block);
        }
    }
    Disk { blocks, files: Vec::new(), free_spaces: Vec::new() }
}

fn move_files_left(disk: &Disk) -> Disk {
    let mut result_disk: Disk = disk.clone();
    for file in disk.files.iter().rev() {
        let opt_free_space = result_disk.free_spaces.iter()
            .find(|fs| file.size <= fs.size && fs.position < file.position)
            .cloned();
        if let Some(free_space) = opt_free_space {
            result_disk.mv(file, &free_space);
        }
    }
    result_disk
}

#[allow(dead_code)]
fn day9_part1() -> isize {
    let filename = "inputs/day9.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let defragmented_disk = compress_free_space(&dbg!(Disk::from(&content)));
    assert!(!defragmented_disk.has_free_space());
    defragmented_disk.calculate_checksum()
}

#[allow(dead_code)]
fn day9_part2() -> isize {
    let filename = "inputs/day9.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let defragmented_disk = move_files_left(&Disk::from(&content));
    defragmented_disk.calculate_checksum()
}

fn main() {
    let result = day9_part2();
    println!("result={result}");
}
