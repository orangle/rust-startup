use std::{fs::File, io::{Read, Write}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct  User {
    pub name : String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self { Self { name, age, gender } }

    pub fn persist(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }  

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;
        Ok(user)
    }
}


impl Default for User {
    fn default() -> Self {
        User::new("no_body".into(), 0, Gender::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wirte_file(){
        let path = "/tmp/user.txt";
        let user = User::default();
        user.persist(path).unwrap();

        let user1 = User::load(path).unwrap();
        assert_eq!(user, user1);
    }
}