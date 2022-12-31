use std::{
    cell::Cell,
    env,
    fs::read_dir,
    path::{Path, PathBuf},
    process,
    rc::Rc,
};

struct Vertex {
    path: PathBuf,
    size: Cell<u64>,
    parent: Option<Rc<Vertex>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = Path::new(args.get(1).unwrap());
    println!("on {:?}", path);

    let root = Rc::new(Vertex {
        path: path.to_path_buf(),
        size: Cell::new(0),
        parent: None,
    });
    let mut stack: Vec<Rc<Vertex>> = vec![Rc::clone(&root)];
    while stack.len() > 0 {
        let parent = stack.pop().unwrap();
        for entry in read_dir(&parent.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                stack.push(Rc::new(Vertex {
                    path: path,
                    size: Cell::new(0),
                    parent: Some(Rc::clone(&parent)),
                }));
            } else {
                println!("{:?}", path);
                let metadata = entry.metadata().unwrap();

                // update all ancestors
                let mut ancestor = Rc::clone(&parent);
                loop {
                    ancestor.size.set(ancestor.size.get() + metadata.len());
                    ancestor = match &ancestor.parent {
                        Some(value) => Rc::clone(&value),
                        None => break
                    };
                }
            }
        }
    }
    println!("{:?}", root.size.get());
}
