#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn get_film_test(){
		assert_eq!(
		  search_by_title(String::from("shrek")).unwrap().Title,
		  "Shrek"
		);
		assert_eq!(
		  search_by_title(String::from("shrek")).unwrap().Year,
		  "2001"
		);
	}
}



use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;



/// Structure for holding information about a Film.
///
/// Currently only holds title and year, however to add more fields from the
/// retrieved JSONs should be added. This should be enough, as serde_json pulls
/// every relevant field when populating an instance of Film.
#[derive(Debug, Serialize, Deserialize)]
pub struct Film{
	pub Title: String,
	pub Year: String
}

/// Searches for and returns a film in the OMDb.
///
/// Sends a request to OMDb for a film with the name `title` and returns a Film
/// object populated with the returned information. Does no input validation,
/// formatting, or case-switching, so can be temperamental. Returns a
/// `reqwest::Error` upon failure.
///
/// # Examples
///
/// To search for the film Shrek:
/// ```
/// use omdbrs;
///
/// let shrek: omdbrs::Film = omdbrs::search_by_title(String::from("shrek"))
///   .unwrap();
///
/// assert_eq!(shrek.Title, "Shrek");
/// ```
pub fn search_by_title(title: String) -> Result<Film, reqwest::Error>{
	let mut data = reqwest::get(
	  &format!("http://www.omdbapi.com/?apikey=21e783b3&t={}", title)[..]
	)?;

	Ok(serde_json::from_str(&data.text().unwrap()).unwrap())
}
