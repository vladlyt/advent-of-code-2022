use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

struct FileSystem<'a> {
    dirs: Vec<Dir<'a>>,
    current_dir: usize,
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        FileSystem {
            dirs: vec![Dir::default()],
            current_dir: 0,
        }
    }

    fn add_file(&mut self, ls_op: &'a LsOp) {
        match ls_op {
            LsOp::File(size) => {
                self.dirs[self.current_dir].size += *size;
                let mut parent = self.dirs[self.current_dir].parent;
                while let Some(parent_idx) = parent {
                    self.dirs[parent_idx].size += *size;
                    parent = self.dirs[parent_idx].parent;
                }
            }
            LsOp::Dir(name) => {
                self.dirs.push(Dir {
                    name: name,
                    size: 0,
                    parent: Some(self.current_dir),
                    dirs: vec![],
                });
                let new_dir_idx = self.dirs.len() - 1;
                self.dirs[self.current_dir].dirs.push(new_dir_idx);
            }
        }
    }

    fn add_files(&mut self, ls_op: &'a Vec<LsOp>) {
        for file in ls_op.iter() {
            self.add_file(file);
        }
    }

    fn change_dir(&mut self, cd_op: &CdOp) {
        match cd_op {
            CdOp::Root => self.current_dir = 0,
            CdOp::Out => {
                self.current_dir = self.dirs[self.current_dir].parent.unwrap();
            }
            CdOp::In(dir_name) => {
                for dir in self.dirs[self.current_dir].dirs.iter() {
                    if self.dirs[*dir].name == *dir_name {
                        self.current_dir = *dir
                    }
                }
            }
        }
    }
}

#[derive(Default, Debug)]
struct Dir<'a> {
    name: &'a str,
    size: usize,
    parent: Option<usize>,
    dirs: Vec<usize>,
}

enum Operation<'a> {
    Cd(CdOp<'a>),
    Ls(Vec<LsOp<'a>>),
}

enum CdOp<'a> {
    Root,
    Out,
    In(&'a str),
}

enum LsOp<'a> {
    File(usize),
    Dir(&'a str),
}

fn parse_file(input: &str) -> IResult<&str, LsOp> {
    let (input, (size, _)) =
        separated_pair(nom::character::complete::u32, tag(" "), take_until("\n"))(input)?;
    Ok((
        input,
        LsOp::File(size as usize),
    ))
}

fn parse_directory(input: &str) -> IResult<&str, LsOp> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, LsOp::Dir(name)))
}

fn parse_ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((parse_file, parse_directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn parse_cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), tag("/"), alpha1))(input)?;
    let op = match dir {
        "/" => Operation::Cd(CdOp::Root),
        ".." => Operation::Cd(CdOp::Out),
        name => Operation::Cd(CdOp::In(name)),
    };
    Ok((input, op))
}
fn parse_commands(input: &str) -> IResult<&str, Vec<Operation>> {
    Ok(separated_list1(newline, alt((parse_ls, parse_cd)))(input)?)
}

pub fn process_part1(input: &str) -> String {
    let (_, operations) = parse_commands(input).unwrap();

    let mut fs = FileSystem::new();

    for op in operations.iter() {
        match op {
            Operation::Cd(cd_op) => fs.change_dir(cd_op),
            Operation::Ls(ls_op) => fs.add_files(ls_op),
        }
    }

    fs.dirs
        .iter()
        .filter_map(|dir| {
            if dir.size < 100000 {
                Some(dir.size)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, operations) = parse_commands(input).unwrap();

    let mut fs = FileSystem::new();

    for op in operations.iter() {
        match op {
            Operation::Cd(cd_op) => fs.change_dir(cd_op),
            Operation::Ls(ls_op) => fs.add_files(ls_op),
        }
    }

    let total_space = 70000000;
    let need_unused = 30000000;

    let used = fs.dirs[0].size;

    let current_free = total_space - used;
    let need_to_free = need_unused - current_free;

    let mut sizes = fs
        .dirs
        .iter()
        .filter_map(|dir| {
            if dir.size > need_to_free {
                Some(dir.size)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    sizes.sort();
    sizes.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
