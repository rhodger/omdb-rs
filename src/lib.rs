#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn get_film_test(){
		assert_eq!(get_film(String::from("shrek")).unwrap().Title, "Shrek");
		assert_eq!(get_film(String::from("shrek")).unwrap().Year, "2001");
	}
}

use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct Film{
	Title: String,
	Year: String
}

fn get_film(title: String) -> Result<Film, reqwest::Error>{
	let mut data = reqwest::get(&format!("http://www.omdbapi.com/?apikey=21e783b3&t={}", title)[..])?;
	Ok(serde_json::from_str(&data.text().unwrap()).unwrap())
}
