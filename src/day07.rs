pub struct Fs {
    dirs: Vec<Dir>,
}

pub struct Dir {
    name: String,
    size: usize,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl Dir {
    pub fn total_size(&self, fs: &Fs) -> usize {
        self.size
            + self
                .children
                .iter()
                .map(|&id| fs.dirs[id].total_size(fs))
                .sum::<usize>()
    }
}

pub fn parse_input(input: &str) -> Fs {
    let mut fs = Fs {
        dirs: vec![Dir {
            name: "/".to_string(),
            size: 0,
            children: Vec::new(),
            parent: None,
        }],
    };

    let mut cwd = 0;

    for line in input.lines() {
        if line.starts_with('$') {
            let mut words = line.split_whitespace().skip(1);

            match (words.next().unwrap(), words.next()) {
                ("cd", Some("/")) => cwd = 0,
                ("cd", Some("..")) => {
                    cwd = fs.dirs[cwd].parent.unwrap();
                }
                ("cd", Some(into)) => {
                    cwd = *fs.dirs[cwd]
                        .children
                        .iter()
                        .find(|&id| fs.dirs[*id].name == into)
                        .unwrap();
                }
                ("ls", None) => (),
                _ => unreachable!(),
            }
        } else {
            match line.split_once(' ').unwrap() {
                ("dir", name) => {
                    fs.dirs.push(Dir {
                        name: name.to_string(),
                        size: 0,
                        children: Vec::new(),
                        parent: Some(cwd),
                    });

                    let id = fs.dirs.len() - 1;
                    fs.dirs[cwd].children.push(id);
                }
                (size, _) => {
                    fs.dirs[cwd].size += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    fs
}

pub fn part_1(fs: &Fs) -> usize {
    fs.dirs
        .iter()
        .filter_map(|dir| {
            let size = dir.total_size(fs);
            (size <= 100000).then_some(size)
        })
        .sum()
}

pub fn part_2(fs: &Fs) -> usize {
    let free = 70000000 - fs.dirs[0].total_size(fs);

    fs.dirs
        .iter()
        .filter_map(|dir| {
            let size = dir.total_size(fs);
            (free + size >= 30000000).then_some(size)
        })
        .min()
        .unwrap()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day07.txt"))),
        1084134
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day07.txt"))),
        6183184
    }
}
