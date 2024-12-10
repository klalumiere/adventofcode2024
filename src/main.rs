use std::{fs};

const FREE_SPACE_TOKEN: isize = -1;

#[derive(Debug)]
struct Disk {
    blocks: Vec<isize>,
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
        Disk { blocks }
    }

    fn has_free_space(self: &Disk) -> bool {
        self.blocks.iter().any(|&block| block == FREE_SPACE_TOKEN)
    }

    fn calculate_checksum(self: &Disk) -> isize {
        self.blocks.iter().enumerate()
            .map(|(i, block)| isize::try_from(i).expect("To fit into isize")*block).sum()
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
    Disk { blocks }
}

#[allow(dead_code)]
fn day9_part1() -> isize {
    let filename = "inputs/day9.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let defragmented_disk = compress_free_space(&Disk::from(&content));
    assert!(!defragmented_disk.has_free_space());
    defragmented_disk.calculate_checksum()
}

fn main() {
    let result = day9_part1();
    println!("result={result}");
}
