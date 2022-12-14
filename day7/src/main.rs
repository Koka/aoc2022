use anyhow::{anyhow, bail, Error as AnyhowError};
use id_tree::{InsertBehavior, Node, NodeId, Tree};
use std::{fs, str::FromStr};

#[derive(Debug)]
enum LogLine {
    Command { name: String, arg: Option<String> },
    Dir { name: String },
    File { name: String, size: u32 },
}

impl FromStr for LogLine {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" ").ok_or(anyhow!("No delimiter"))?;
        match parts {
            ("$", cmd) => {
                let splitted = cmd.split_once(' ');
                if let Some((cmd_name, arg)) = splitted {
                    Ok(LogLine::Command {
                        name: cmd_name.to_owned(),
                        arg: Some(arg.to_owned()),
                    })
                } else {
                    Ok(LogLine::Command {
                        name: cmd.to_owned(),
                        arg: None,
                    })
                }
            }
            ("dir", name) => Ok(LogLine::Dir {
                name: name.to_owned(),
            }),
            (size, name) => Ok(LogLine::File {
                name: name.to_owned(),
                size: size.parse()?,
            }),
        }
    }
}

fn find_parent<T>(tree: &Tree<T>, node_id: &NodeId) -> Result<Option<NodeId>, AnyhowError> {
    Ok(tree.ancestor_ids(node_id)?.next().map(|d| d.clone()))
}

fn get_data<T>(tree: &Tree<T>, node_id: &NodeId) -> Result<T, AnyhowError>
where
    T: Clone,
{
    let me = tree.get(node_id)?;
    Ok(me.data().clone())
}

fn add_child<T>(tree: &mut Tree<T>, node_id: &NodeId, data: T) -> Result<NodeId, AnyhowError> {
    let node = Node::new(data);
    tree.insert(node, InsertBehavior::UnderNode(node_id))
        .map_err(|e| e.into())
}

fn add_size_to_parent(tree: &mut Tree<(String, u32)>, node_id: &NodeId) -> Result<(), AnyhowError> {
    let data = get_data(tree, node_id)?;

    if let Some(parent_id) = find_parent(tree, node_id)? {
        let parent = tree.get_mut(&parent_id)?;
        let parent_data = parent.data();
        let new_data = (parent_data.0.clone(), data.1 + parent_data.1);
        parent.replace_data(new_data);
    }

    Ok(())
}

fn main() -> Result<(), AnyhowError> {
    let input = fs::read_to_string("./input.txt")?;

    let log = input.lines().filter_map(|s| s.parse::<LogLine>().ok());

    let mut tree: Tree<(String, u32)> = Tree::new();
    let root_id = tree.insert(Node::new(("/".to_owned(), 0)), InsertBehavior::AsRoot)?;
    let mut current = root_id.clone();

    for line in log {
        match line {
            LogLine::Command {
                name,
                arg: Some(dir),
            } if name == "cd" => match dir.as_str() {
                "/" => {
                    current = root_id.clone();
                }
                ".." => {
                    current = find_parent(&tree, &current)?
                        .ok_or(anyhow!("No parent for {:?}", current))?;
                }
                path => {
                    current = add_child(&mut tree, &current, (path.to_owned(), 0))?;
                }
            },
            LogLine::Command { name, arg: _ } if name == "ls" => {}
            LogLine::Command { name, arg: _ } => bail!("Unknown command {}", name),
            LogLine::Dir { name: _ } => {}
            LogLine::File { name, size } => {
                add_child(&mut tree, &current, (name.to_owned(), size))?;
            }
        }
    }

    for node_id in tree.traverse_post_order_ids(&root_id)? {
        add_size_to_parent(&mut tree, &node_id)?;
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{}", s);

    let total_space = 70000000u32;
    let required_space = 30000000u32;

    let occupied_space = tree.get(&root_id)?.data().1;
    let free_space = total_space - occupied_space;
    let left_to_free = required_space - free_space;

    dbg!(
        total_space,
        required_space,
        occupied_space,
        free_space,
        left_to_free
    );

    let mut min_dir = tree.get(&root_id)?.data();
    for node in tree.traverse_post_order(&root_id)? {
        let is_dir = !node.children().is_empty();
        let size = node.data().1;
        if is_dir && size >= left_to_free {
            println!("Big enough dir {:?}", node.data());
            if size < min_dir.1 {
                min_dir = node.data();
            }
        }
    }

    println!("Smallest big enough dir: {:?}", min_dir);

    Ok(())
}
