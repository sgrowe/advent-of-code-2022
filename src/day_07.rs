use crate::{solution::Solution, utils::parse_as};

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<usize, usize> {
    let mut path = CurPath::default();

    let mut part_one = 0;

    let mut all_dirs = AllDirSizes::new();

    for line in input.lines().map(OutputLine::from) {
        match line {
            OutputLine::Cd("..") => {
                let (name, size) = path.pop().unwrap();

                all_dirs.add_folder(name, size);

                if size <= 100000 {
                    part_one += size;
                }
            }
            OutputLine::Cd(dir) => path.push(dir),
            OutputLine::Ls => {}
            OutputLine::File { size, name } => path.add_file_size(size),
            OutputLine::Dir(dir) => {}
        }
    }

    while let Some((name, size)) = path.pop() {
        all_dirs.add_folder(name, size);

        if size <= 100000 {
            part_one += size
        }
    }

    let total_disk_space = 70000000;
    let total_free_space_required = 30000000;

    let current_free_space = total_disk_space - all_dirs.total_space_used();

    let additional_space_required = total_free_space_required - current_free_space;

    let part_two = all_dirs.smallest_dir_size_bigger_than(additional_space_required);

    Solution { part_one, part_two }
}

#[derive(Debug, Default)]
struct CurPath<'a> {
    i: usize,
    path: [(&'a str, usize); 15],
}

impl<'a> CurPath<'a> {
    pub fn push(&mut self, name: &'a str) {
        self.i += 1;
        self.path[self.i] = (name, 0);
    }

    pub fn add_file_size(&mut self, size: usize) {
        for x in 0..=self.i {
            self.path[x].1 += size;
        }
    }

    pub fn pop(&mut self) -> Option<(&'a str, usize)> {
        if self.i == 0 {
            return None;
        }

        let x = self.path[self.i];
        self.i -= 1;
        Some(x)
    }
}

enum OutputLine<'a> {
    Cd(&'a str),
    Ls,
    File { size: usize, name: &'a str },
    Dir(&'a str),
}

impl<'a> From<&'a str> for OutputLine<'a> {
    fn from(s: &'a str) -> Self {
        if s.starts_with('$') {
            if s == "$ ls" {
                return OutputLine::Ls;
            }

            if let Some(dir) = s.strip_prefix("$ cd ") {
                return OutputLine::Cd(dir);
            }
        } else {
            if let Some(dir) = s.strip_prefix("dir ") {
                return OutputLine::Dir(dir);
            } else {
                let (size, name) = s.split_once(' ').unwrap();

                return OutputLine::File {
                    size: parse_as(size),
                    name,
                };
            }
        }

        panic!("Unknown format");
    }
}

struct AllDirSizes<'a> {
    i: usize,
    dirs: [(&'a str, usize); 200],
}

impl<'a> AllDirSizes<'a> {
    pub fn new() -> Self {
        Self {
            i: 0,
            dirs: [("", 0); 200],
        }
    }

    pub fn add_folder(&mut self, name: &'a str, size: usize) {
        self.dirs[self.i] = (name, size);
        self.i += 1;
    }

    pub fn total_space_used(&self) -> usize {
        (self.dirs[..self.i])
            .into_iter()
            .copied()
            .find(|&(name, _)| name == "/")
            .unwrap()
            .1
    }

    pub fn smallest_dir_size_bigger_than(&self, min_size: usize) -> usize {
        (self.dirs[..self.i])
            .into_iter()
            .copied()
            .filter(|&(_, size)| size >= min_size)
            .min_by_key(|&(_, size)| size)
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn part_one() {
        assert_eq!(solution(SAMPLE.trim()).part_one, 95437);

        assert_eq!(solution(&read_input(7)).part_one, 1477771);
    }

    #[test]
    fn part_two() {
        assert_eq!(solution(SAMPLE.trim()).part_two, 24933642);

        assert_eq!(solution(&read_input(7)).part_two, 3579501);
    }
}
