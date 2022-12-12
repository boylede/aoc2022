use id_tree::*;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A];
pub const INPUT: &str = include_str!("../../input/day7.txt");

pub const INPUT_A: &str = "$ cd /
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
7214296 k";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let (filesystem, _) = input
        .lines()
        .map(|s| -> Line {
            let first = s.chars().nth(0).unwrap();
            match first {
                '$' => {
                    if s == "$ ls" {
                        // ignore ls command
                        Line::LS
                    } else {
                        // cd command "$ cd dirname"
                        let (_, dirname) = s.split_at(5);
                        Line::CD(dirname)
                    }
                }
                'd' => {
                    let (_, dirname) = s.split_at(4);
                    Line::Entry(Entry::Directory(None, dirname))
                }
                _ => {
                    let (size, name) = s.split_once(' ').unwrap();
                    let size = size.parse().unwrap();
                    Line::Entry(Entry::File(size, name))
                }
            }
        })
        .fold(
            (TreeBuilder::<Entry>::new().build(), None::<NodeId>),
            |(mut tree, parent), command| {
                let parent_name = parent
                .as_ref()
                    .and_then(|id| tree.get(id).ok())
                    .map(|node| node.data())
                    .and_then(|entry| {
                        if let Entry::Directory(None, name) = entry {
                            Some(name)
                        } else {
                            None
                        }
                    });
                // todo!()
                 match command {
                   Line::LS => {
                       let Entry::Directory(_, parent_name) = tree.get(parent.as_ref().unwrap()).unwrap().data() else {
                           panic!("");
                       };
                       (tree, parent)
                   }
                   Line::CD(dirname) => match parent {
                       Some(parent) => {
                           if dirname == ".." {

                                let next_dir = tree.get(&parent).unwrap().parent().unwrap().clone();
                                (tree, Some(next_dir))
                           } else {

                            let next_dir = tree
                                .children_ids(&parent)
                                .unwrap()
                                .find(|child_id| {
                                    let Ok(test_node) = tree.get(child_id) else {
                                        return false;
                                    };
                                    let Entry::Directory(_, test_dirname) = test_node.data() else {
                                        return false;
                                    };

                                    dirname == *test_dirname
                                })
                                .cloned().unwrap();
                            (tree, Some(next_dir))
                           }
                       }
                       None => {

                           let root = tree
                               .insert(Node::new(Entry::Directory(None, dirname)), InsertBehavior::AsRoot)
                               .unwrap();
                           (tree, Some(root))
                       }
                   },
                   Line::Entry(next) => {
                       let behaviour = InsertBehavior::UnderNode(parent.as_ref().unwrap());
                       match next {
                           Entry::Directory(_, dirname) => {

                               tree.insert(Node::new(next), behaviour).unwrap();
                           }
                           Entry::File(size, name) => {

                               tree.insert(Node::new(next), behaviour).unwrap();
                           }
                       };
                       (tree, parent)
                   }
               }
            }
        );
    let mut filesystem = filesystem.clone();
    let traversal_list: Vec<NodeId> = filesystem
        .traverse_post_order_ids(filesystem.root_node_id().unwrap())
        .unwrap()
        .collect();
    for node_id in traversal_list.into_iter() {
        let node = filesystem.get(&node_id).unwrap();
        let children: Vec<NodeId> = node.children().clone();
        match node.data() {
            Entry::Directory(None, name) => {
                let inner_size: usize = children
                    .iter()
                    .map(|c_id| match filesystem.get(c_id).unwrap().data() {
                        Entry::Directory(Some(size), _) => *size,
                        Entry::Directory(None, _) => panic!("we shouldn't get here due to traversal order"),
                        Entry::File(size, _) => *size,
                    })
                    .sum();

                {

                    *filesystem.get_mut(&node_id).unwrap().data_mut() =
                        Entry::Directory(Some(inner_size), name);
                }
            }
            Entry::Directory(Some(_), _) => (),
            Entry::File(_, _) => (),
        }
    }

    let parta: usize = filesystem
        .traverse_post_order(filesystem.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| -> Option<usize> {
            let data = node.data();
            match data {
                Entry::Directory(size, _) => {
                    let Some(size) = size else {
                        return None;
                    };
                    if *size <= 100_000 {
                        Some(*size)
                    } else {
                        None
                    }
                }
                Entry::File(_, _) => None,
            }
        })
        .sum();

    let Entry::Directory(Some(root_dir_size), _) = filesystem.get(filesystem.root_node_id().unwrap()).unwrap().data() else {
            unreachable!();
        };

    let free_space = 70_000_000 - *root_dir_size;
    let required_space = 30_000_000 - free_space;

    let partb = filesystem
        .traverse_post_order(filesystem.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| -> Option<usize> {
            match node.data() {
                Entry::Directory(Some(size), _) => Some(*size),
                _ => None,
            }
        })
        .filter(|num| *num >= required_space)
        .min()
        .unwrap();
    (format!("{parta}"), format!("{partb}"))
}

enum Line<'a> {
    LS,
    CD(&'a str),
    Entry(Entry<'a>),
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Entry<'a> {
    Directory(Option<usize>, &'a str),
    File(usize, &'a str),
}
