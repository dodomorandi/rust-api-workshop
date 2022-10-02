use workshop::swapi::Person;

pub fn luke() -> Person {
    Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    }
}

pub fn yoda() -> Person {
    Person {
        name: "Yoda".to_string(),
        height: "66".to_string(),
    }
}

pub fn yaddle() -> Person {
    Person {
        name: "Yaddle".to_string(),
        height: "61".to_string(),
    }
}

pub fn arvel() -> Person {
    Person {
        name: "Arvel Crynyd".to_string(),
        height: "unknown".to_string(),
    }
}
