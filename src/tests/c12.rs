use crate::tasks::c12_object_oriented_programming::{
    Account,
    BackTo2007,
    BritishPigeon,
    Converter,
    IntToHexConverter,
    Lion,
    StringToIntConverter,
    Zoo,
};

#[test]
fn test_zoo() {
    let mut zoo = Zoo::new();

    zoo.add_animal(Box::new(Lion::new("Simba", 0.0, 0.0)));
    zoo.add_animal(Box::new(BritishPigeon::new("Percy", 2.0, 3.0)));
    assert_eq!(
        vec![
            "Roar!".to_string(),
            "Oi mate! Bloody hell I love fish'n'chips brof!".to_string()
        ],
        zoo.make_all_noises()
    );

    zoo.move_all(1.0, -1.0);
    assert_eq!(vec![("Simba", (1.0, -1.0)), ("Percy", (3.0, 2.0))], zoo.positions());
}

#[test]
fn test_back_to_2007() {
    let account = Account::new("NAGIBATOR", 1999);
    assert_eq!("★彡Xx_NAGIBATOR1999_xX彡★", account.cringify());
}

#[test]
fn test_converter() {
    let stoi_converter = StringToIntConverter;
    assert_eq!(255, stoi_converter.convert("255".to_string()));

    let itoh_converter = IntToHexConverter;
    assert_eq!("ff", itoh_converter.convert(255));
}
