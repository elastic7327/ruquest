

pub struct Ruquests {
    pub url: String,
}

impl Ruquests {
    pub fn new() -> Ruquests {
        Ruquests{url: String::new()}
    }
    
}


#[test]
fn test_ruquests() {
    let mut ru = Ruquests::new();
    ru.url = "www.google.com".to_string();
    assert_eq!("www.google.com", ru.url);
}
