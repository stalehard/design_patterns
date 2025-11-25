mod abstract_factory;
mod adapter;
mod bridge;
mod builder;
mod factory_method;
mod singleton;

use factory_method::{AnimalFactory, AnimalFactoryRegistry, BirdFactory, CatFactory, DogFactory};

fn main() {
    println!("=== Factory Method Pattern Demo ===\n");

    println!("1. Using specific factories:");
    let dog_factory = DogFactory;
    let cat_factory = CatFactory;
    let bird_factory = BirdFactory;

    // Create animals using specific factories
    dog_factory.create_and_introduce("Buddy".to_string());

    cat_factory.create_and_introduce("Whiskers".to_string());

    bird_factory.create_and_introduce("Tweety".to_string());

    println!("2. Using factory registry:");
    let registry = AnimalFactoryRegistry::new();

    let animals_to_create = vec![
        ("dog", "Rex"),
        ("cat", "Fluffy"),
        ("bird", "Charlie"),
        ("dog", "Max"),
    ];

    for (animal_type, name) in animals_to_create {
        registry.create_and_introduce(animal_type, name.to_string());
    }

    println!("3. Available animal types in registry:");
    let available_types = registry.available_types();
    println!("Available types: {}", available_types.join(", "));

    let animal_specs = vec![
        ("dog", "Luna"),
        ("cat", "Shadow"),
        ("bird", "Rio"),
        ("dog", "Cooper"),
        ("fish", "Nemo"),
    ];

    let mut animals = Vec::new();
    for (animal_type, name) in animal_specs {
        let animal = registry.create_animal(animal_type, name.to_string());
        animals.push(animal);
    }

    for animal in &animals {
        animal.speak();
    }
}
