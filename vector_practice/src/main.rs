#[derive(Debug)]
struct File {
    name: String
}
#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}
impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name: name,
            contents: Vec::new()
        }
    }
    fn create_file(&mut self, name: String) {
        let new_file = File {
            name
        };
        self.contents.push(new_file);
    }
    fn remove_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }
    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}
fn main() {
    let mut dir = Folder::new("fold".to_string());
    dir.create_file("file1".to_string());
    dir.create_file("file2".to_string());

    println!("{:?}", dir);

    dir.remove_file(1);
    println!("{:?}", dir);
    dir.remove_file(0);
    let file = dir.get_file(0);
    
    match file {
        Option::Some(f) => {
            println!("File: {:?}", f);
        }
        Option::None => println!("Folder is empty")
    }

}
