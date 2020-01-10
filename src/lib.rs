#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn get_film_test(){
		assert_eq!(get_film().unwrap().Title, "Shrek");
		assert_eq!(get_film().unwrap().Year, "2001");
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

fn get_film() -> Result<Film, reqwest::Error>{
	let mut data = reqwest::get("http://www.omdbapi.com/?apikey=21e783b3&t=shrek")?;
	Ok(serde_json::from_str(&data.text().unwrap()).unwrap())
}
