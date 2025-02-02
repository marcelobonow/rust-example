use crate::utils;

pub struct Book {
    pub title: String,
    pub author: String,
    pub publication_date: chrono::NaiveDate,
}

impl Book {
    pub fn serialize(&self) -> String {
        let name_parsed = utils::parse_string(&self.title);
        let author = utils::parse_string(&self.author);
        let publication_date = utils::parse_string(&self.publication_date.to_string());
        let result = format!("name:{name_parsed};author:{author};date:{publication_date}\n");
        return result;
    }
}
