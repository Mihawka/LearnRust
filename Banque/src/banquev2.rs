
pub fn started() {
    let mut banque: Banque = Banque::new();

    banque.money += 10;
    banque.money -= 10;

    banque.display();
}

struct Banque {
    pub money: i32,
}
impl Banque {
    fn new() -> Banque {
        Banque { money: 0 }
    }
    fn display(&self) {
        println!("{}", self.money);
    }
}