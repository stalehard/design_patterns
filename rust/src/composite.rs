pub trait FileSystemComponent {
    fn name(&self) -> &str;
    fn calculate_size(&self) -> usize;
}

pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

impl FileSystemComponent for File {
    fn name(&self) -> &str {
        &self.name
    }

    fn calculate_size(&self) -> usize {
        self.size
    }
}

pub struct Directory {
    name: String,
    children: Vec<Box<dyn FileSystemComponent>>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box<dyn FileSystemComponent>) {
        self.children.push(child);
    }
}

impl FileSystemComponent for Directory {
    fn name(&self) -> &str {
        &self.name
    }

    fn calculate_size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.calculate_size())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_system_component_composite_structure() {
        // Create leaf components
        let file1 = File::new("file1.txt".to_string(), 10);
        let file2 = File::new("file2.txt".to_string(), 20);
        let file3 = File::new("file3.log".to_string(), 5);

        // Verify leaf properties
        assert_eq!(file1.name(), "file1.txt");
        assert_eq!(file1.calculate_size(), 10);

        // Create composite components
        let mut root_dir = Directory::new("root".to_string());
        let mut sub_dir = Directory::new("subdir".to_string());

        // Build the tree structure
        sub_dir.add_child(Box::new(file2));
        sub_dir.add_child(Box::new(file3));

        root_dir.add_child(Box::new(file1));
        root_dir.add_child(Box::new(sub_dir));

        // Verify composite properties
        // The size of the root directory should be the sum of all components.
        // file1 (10) + sub_dir (file2 (20) + file3 (5)) = 35
        assert_eq!(root_dir.name(), "root");
        assert_eq!(root_dir.calculate_size(), 35);
    }
}
