use std::collections::HashMap;
//========================================================================================
pub fn started() {
    println!("Hello Banque V3");
    let mut banque: Banque = Banque::new();
    let marc: Human = Human::new(String::from("Marc"));
    let other: Human = Human::new(String::from("Other"));

    banque.create_account(&marc);
    banque.create_account(&other);

    banque.add_money(&marc, 50);
    banque.display_money(&marc);
}
//========================================================================================
struct Human {
    pub name: String,
}
impl Human {
    fn new(name: String) -> Human {
        Human {name: name}
    }
}
//========================================================================================
struct Banque {
    pub account: HashMap<String, i32>
}
impl Banque {
    fn new() -> Banque {
        Banque{ account: HashMap::new() }
    }
    fn create_account(&mut self, human: &Human) {
        self.account.insert(human.name.clone(), 0);
    }
    fn delete_account(&mut self, human: &Human) {
        self.account.remove(&human.name);
    }
    fn add_money(&mut self, human: &Human, value: i32) {
        if !self.account.contains_key(&human.name) {
            println!("This account does not exist!");
            return;
        }
        *self.account.get_mut(&human.name).unwrap() += value;
        println!("Opération successfully!")
    }
    fn display_money(&self, human: &Human) {
        println!("Solde de {}: {}", human.name.clone(), self.account[&human.name]);
    }
}
//========================================================================================