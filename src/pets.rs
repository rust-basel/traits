use crate::make_sound::MakeSound;

struct Dog {
    is_happy: bool,
    is_chubby: bool,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            is_happy: false,
            is_chubby: true,
        }
    }

    fn bark(&self) -> String {
        "Woof".to_string()
    }

    fn pet(&mut self) {
        todo!()
    }
}

impl MakeSound for Dog {
    fn make_sound(&self) -> String {
        todo!()
    }
}

struct Cat;

impl MakeSound for Cat {
    fn make_sound(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::make_sound::MakeSound;
    use crate::pets::{Cat, Dog};

    #[test]
    fn my_dog_is_chubby() {
        // given
        let my_dog = Dog::new();

        // when, then
        assert!(my_dog.is_chubby);
    }

    #[test]
    fn petting_a_dog_makes_it_happy() {
        // given
        let mut dog = Dog::new();

        // when
        dog.pet();

        // then
        assert!(dog.is_happy);
    }

    #[test]
    fn pets_make_funny_sounds() {
        // given
        let first_pet = Box::new(Dog::new());
        let second_pet = Box::new(Cat);
        let zoo: Vec<Box<dyn MakeSound>> = vec![first_pet, second_pet];

        // when
        let sound_records: Vec<String> = zoo.iter().map(|pet| pet.make_sound()).collect();

        // then
        assert_eq!(sound_records[0], "Woof".to_string());
        assert_eq!(sound_records[1], "Miauw".to_string());
    }
}
