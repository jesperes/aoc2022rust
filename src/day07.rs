use std::collections::HashMap;
use std::path::PathBuf;

type SubDirMap = HashMap<PathBuf, Vec<PathBuf>>;
type SizeMap = HashMap<PathBuf, i64>;

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input07.txt");
    let s = String::from_utf8_lossy(buf);
    let p1: i64;
    let p2: i64;
    let mut cwd: PathBuf = PathBuf::new();
    let mut sub_dirs: SubDirMap = HashMap::new();
    let mut sizes: SizeMap = HashMap::new();

    for line in s.trim().split("\n") {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words[..] {
            ["$", "cd", "/"] => {
                cwd = PathBuf::new();
            }
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", dir] => {
                cwd.push(dir);
            }
            ["$", "ls"] => {
                // nop
            }
            ["dir", dir] => {
                let mut full_dir = cwd.clone();
                full_dir.push(dir);
                sub_dirs.entry(cwd.clone()).or_insert(vec![]).push(full_dir);
            }
            [size, _file] => {
                *sizes.entry(cwd.clone()).or_insert(0) += size.parse::<i64>().unwrap();
            }
            _ => panic!(),
        }
    }

    compute_sizes(PathBuf::new(), &sub_dirs, &mut sizes);

    p1 = sizes.values().filter(|v| **v <= 100_000).sum();

    const MAX_FILE_SIZE: i64 = 40_000_000;
    let total_used = sizes.get(&PathBuf::new()).unwrap();

    p2 = *sizes
        .values()
        .filter(|v| total_used - **v <= MAX_FILE_SIZE)
        .min()
        .unwrap();

    (p1, p2)
}

fn compute_sizes(root: PathBuf, sub_dirs: &SubDirMap, sizes: &mut SizeMap) {
    if let Some(list) = sub_dirs.get(&root) {
        for d in list {
            compute_sizes(d.clone(), sub_dirs, sizes);
            *sizes.entry(root.clone()).or_insert(0) += *sizes.get(d).unwrap();
        }
    }
}
