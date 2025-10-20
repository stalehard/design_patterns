pub trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
    fn species(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} barks: Woof!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "Dog"
    }
}

#[derive(Debug, Clone)]
pub struct Cat {
    name: String,
}

impl Cat {
    pub fn new(name: String) -> Self {
        Cat { name }
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} meows: Meow!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "Cat"
    }
}

#[derive(Debug, Clone)]
pub struct Bird {
    name: String,
}

impl Bird {
    pub fn new(name: String) -> Self {
        Bird { name }
    }
}

impl Animal for Bird {
    fn speak(&self) {
        println!("{} chirps: Tweet!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "Bird"
    }
}

#[derive(Debug, Clone)]
pub struct Unknown {
    name: String,
}

impl Unknown {
    pub fn new(name: String) -> Self {
        Unknown { name }
    }
}

impl Animal for Unknown {
    fn speak(&self) {
        println!("{} says: ???", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "Unknown"
    }
}

pub trait AnimalFactory {
    fn create_animal(&self, name: String) -> Box<dyn Animal>;

    fn create_and_introduce(&self, name: String) {
        let animal = self.create_animal(name);
        println!("Created a new {} named {}", animal.species(), animal.name());
        animal.speak();
    }
}

pub struct DogFactory;

impl AnimalFactory for DogFactory {
    fn create_animal(&self, name: String) -> Box<dyn Animal> {
        Box::new(Dog::new(name))
    }
}

pub struct CatFactory;

impl AnimalFactory for CatFactory {
    fn create_animal(&self, name: String) -> Box<dyn Animal> {
        Box::new(Cat::new(name))
    }
}

pub struct BirdFactory;

impl AnimalFactory for BirdFactory {
    fn create_animal(&self, name: String) -> Box<dyn Animal> {
        Box::new(Bird::new(name))
    }
}

pub struct UnknownFactory;

impl AnimalFactory for UnknownFactory {
    fn create_animal(&self, name: String) -> Box<dyn Animal> {
        Box::new(Unknown::new(name))
    }
}

pub struct AnimalFactoryRegistry {
    factories: std::collections::HashMap<String, Box<dyn AnimalFactory>>,
}

impl AnimalFactoryRegistry {
    pub fn new() -> Self {
        let mut registry = AnimalFactoryRegistry {
            factories: std::collections::HashMap::new(),
        };

        registry.register("dog".to_string(), Box::new(DogFactory));
        registry.register("cat".to_string(), Box::new(CatFactory));
        registry.register("bird".to_string(), Box::new(BirdFactory));
        registry.register("unknown".to_string(), Box::new(UnknownFactory));

        registry
    }

    pub fn register(&mut self, animal_type: String, factory: Box<dyn AnimalFactory>) {
        self.factories.insert(animal_type.to_lowercase(), factory);
    }

    pub fn create_animal(&self, animal_type: &str, name: String) -> Box<dyn Animal> {
        let animal_type = animal_type.to_lowercase();
        match self.factories.get(&animal_type) {
            Some(factory) => factory.create_animal(name),
            None => self
                .factories
                .get(&"unknown".to_string())
                .unwrap()
                .create_animal(name),
        }
    }

    pub fn available_types(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }

    pub fn create_and_introduce(&self, animal_type: &str, name: String) {
        let animal_type = animal_type.to_lowercase();
        match self.factories.get(&animal_type) {
            Some(factory) => factory.create_and_introduce(name),
            None => self
                .factories
                .get(&"unknown".to_string())
                .unwrap()
                .create_and_introduce(name),
        }
    }
}

impl Default for AnimalFactoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_factory() {
        let factory = DogFactory;
        let dog = factory.create_animal("Buddy".to_string());
        assert_eq!(dog.name(), "Buddy");
        assert_eq!(dog.species(), "Dog");
    }

    #[test]
    fn test_cat_factory() {
        let factory = CatFactory;
        let cat = factory.create_animal("Whiskers".to_string());
        assert_eq!(cat.name(), "Whiskers");
        assert_eq!(cat.species(), "Cat");
    }

    #[test]
    fn test_registry() {
        let registry = AnimalFactoryRegistry::new();
        let dog = registry.create_animal("dog", "Rex".to_string());
        assert_eq!(dog.species(), "Dog");

        let cat = registry.create_animal("CAT", "Fluffy".to_string());
        assert_eq!(cat.species(), "Cat");
    }

    #[test]
    fn test_unknown_animal_type() {
        let registry = AnimalFactoryRegistry::new();
        let result = registry.create_animal("elephant", "Dumbo".to_string());
        assert_eq!(result.species(), "Unknown");
    }
}
