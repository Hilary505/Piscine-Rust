#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback, 
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}


pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let jacket;
    if formality_level.is_none() {
        jacket = Jacket::Flowers;
    }else if formality_level.unwrap() > 0 {
        jacket = Jacket::White;
    }else {
        jacket = Jacket::Black;
    }

    let hat;
    if formality_level.is_none() && invitation_message.is_err() {
        hat = Hat::Baseball;
    }else if invitation_message.is_ok() {
        hat = Hat::Fedora;
    }else {
        hat = Hat::Snapback
    }
    Outfit {jacket, hat}
}