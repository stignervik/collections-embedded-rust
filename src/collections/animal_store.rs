trait Animal {
    fn sound(&self) -> &str;
}

struct Dog {
    name: u16,
}

impl Animal for Dog {
    fn sound(&self) -> &str {
        "woof!"
        // format!("{} says: woof!", self.name)
    }
}

struct Cat {
    name: u16,
}

impl Animal for Cat {
    fn sound(&self) -> &str {
        "meow!"
    }
}

enum AnimalEnum {
    Dog(Dog),
    Cat(Cat),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal() {
        let mut animals: Vec<AnimalEnum> = Vec::new();
        animals.push(AnimalEnum::Dog(Dog { name: 0 }));
        animals.push(AnimalEnum::Cat(Cat { name: 1 }));
        for animal in animals.iter() {
            match animal {
                AnimalEnum::Dog(dog) => println!("{}", dog.sound()),
                AnimalEnum::Cat(cat) => println!("{}", cat.sound()),
            }
        }
    }
}

/*
trait Animal {
  fn sound(&self) -> String;
}

struct Dog {
  name: String,
}

impl Animal for Dog {
  fn sound(&self) -> String {
    format!("{} says: woof!", self.name)
  }
}

struct Cat {
  name: String,
}

impl Animal for Cat {
  fn sound(&self) -> String {
    format!("{} says: meow!", self.name)
  }
}

struct AnimalCollection<T: Animal> {
  animals: Vec<T>,
}

impl<T: Animal> AnimalCollection<T> {
  fn new() -> Self {
    AnimalCollection {
      animals: Vec::new(),
    }
  }

  fn add(&mut self, animal: T) {
    self.animals.push(animal);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_animal_collection() {
    let mut animal_collection = AnimalCollection::new();
    animal_collection.add(Dog {
      name: String::from("Rusty"),
    });
    /*
    animal_collection.add(Cat {
      name: String::from("Misty"),
        });
        */

        let mut sounds = Vec::new();
        for animal in &animal_collection.animals {
          sounds.push(animal.sound());
        }

        assert_eq!(sounds, vec!["Rusty says: woof!"]);
      }
    }
    */

/*
    trait Animal {
      fn sound(&self) -> String;
    }

    struct Dog {
      name: String,
    }

    impl Animal for Dog {
      fn sound(&self) -> String {
        format!("{} says: woof!", self.name)
      }
    }

    struct Cat {
  name: String,
}

impl Animal for Cat {
  fn sound(&self) -> String {
    format!("{} says: meow!", self.name)
  }
}

struct AnimalCollection<T> {
  animals: Vec<T>,
}

impl<T> AnimalCollection<T> {
  fn new() -> Self {
      AnimalCollection { animals: Vec::new() }
  }

  fn add(&mut self, animal: T) {
      self.animals.push(animal);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_animal_collection() {
      let mut animal_collection = AnimalCollection::new();
      animal_collection.add(Dog {
          name: String::from("Rusty"),
      });
      animal_collection.add(Cat {
          name: String::from("Misty"),
      });

      let mut sounds = Vec::new();
      for animal in &animal_collection.animals {
          sounds.push(animal.sound());
      }

      assert_eq!(sounds, vec!["Rusty says: woof!", "Misty says: meow!"]);
  }
}
*/

/*

*/

/*
trait Animal {
  fn sound(&self) -> String;
}

struct Dog {
  name: String,
}

impl Animal for Dog {
  fn sound(&self) -> String {
    format!("{} says: woof!", self.name)
  }
}

struct Cat {
  name: String,
}

impl Animal for Cat {
  fn sound(&self) -> String {
    format!("{} says: meow!", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_animal() {
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Dog {
      name: String::from("Rusty"),
    }));
    animals.push(Box::new(Cat {
      name: String::from("Misty"),
    }));
    for animal in animals.iter() {
      println!("{}", animal.sound());
    }
    }
  }
*/
