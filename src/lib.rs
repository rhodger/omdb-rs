//! A library for interacting with the OMDb in Rust.
//!
//! Interactions are mostly through the `Film` object, which is constructed
//! using some information usable in an OMDb search, and which can then be
//! interacted with through a series of methods.

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn search_by_title_test(){
		assert_eq!(
		  search_by_title(String::from("shrek")).unwrap().Title,
		  "Shrek"
		);
		assert_eq!(
		  search_by_title(String::from("shrek")).unwrap().Year,
		  "2001"
		);
	}

	#[test]
	fn search_by_id_test(){
		assert_eq!(
		  search_by_id(String::from("tt0126029")).unwrap().Title,
		  "Shrek"
		);
		assert_eq!(
		  search_by_id(String::from("tt0126029")).unwrap().Year,
		  "2001"
		);
	}

	#[test]
	fn from_title_test(){
		let film: Film = Film::from_title(String::from("Shrek")).unwrap();

		assert_eq!(film.Title, "Shrek");
		assert_eq!(film.Year, "2001");
		assert_eq!(film.Runtime, "90 min");

		assert!(Film::from_title(String::from("gobbeldygookasdfblu")).is_err());
	}

	#[test]
	fn from_id_test(){
		let film: Film = Film::from_id(String::from("tt0126029")).unwrap();

		assert_eq!(film.Title, "Shrek");
		assert_eq!(film.Year, "2001");

		assert!(Film::from_id(String::from("gobbeldygookasdfblur")).is_err());
	}

	#[test]
	fn film_get_test(){
		let film: Film = Film{
			Title: String::from("Shrek"),
			Year: String::from("2001"),
			Runtime: String::from("90 min")
		};

		assert_eq!(film.get_title(), "Shrek");
		assert_eq!(film.get_year(), "2001");
		assert_eq!(film.get_runtime(), "90 min");
	}
}


use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use custom_error::custom_error;


custom_error!{pub FilmError
	FilmNotFound = "No film matching the given criteria was found"
}


/// Structure for holding information about a Film.
///
/// Currently only holds title and year, however to add more fields from the
/// retrieved JSONs should be added. This should be enough, as serde_json pulls
/// every relevant field when populating an instance of Film.
///
/// # Examples
///
/// To create a Film object representing the film Shrek:
/// ```
/// use omdbrs::Film;
/// 
/// let film: Film = Film::from_title(String::from("Shrek")).unwrap();
///
/// assert_eq!(film.get_title(), "Shrek");
/// ```
/// In practice, `unwrap()` should not be used as a `FilmError` may be returned.
#[derive(Debug, Serialize, Deserialize)]
pub struct Film{
	Title: String,
	Year: String,
	Runtime: String
}


impl Film{
	/// Constructor for a Film object using a film's title.
	///
	/// Creates a Film object using the result of an OMDb query using the given
	/// title. If no matching film is found, a `FilmError` is returned instead.
	pub fn from_title(title: String) -> Result<Film, FilmError>{
		let film: Film = match search_by_title(title){
			Ok(x) => x,
			Err(e) => return Err(FilmError::FilmNotFound)
		};

		Ok(film)
	}

	/// Constructor for a Film object using a film's IMDB id.
	///
	/// Creates a Film object using the result of an OMDb query using the given
	/// id. If no matching film is found, a `FilmError` is returned instead.
	pub fn from_id(id: String) -> Result<Film, FilmError>{
		let film: Film = match search_by_id(id){
			Ok(x) => x,
			Err(e) => return Err(FilmError::FilmNotFound)
		};

		Ok(film)
	}

	pub fn get_title(&self) -> String{ String::from(&self.Title) }

	pub fn get_year(&self) -> String{ String::from(&self.Year) }

	pub fn get_runtime(&self) -> String{ String::from(&self.Runtime) }
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
/// ``` ignore
/// use omdbrs;
///
/// let shrek: omdbrs::Film = omdbrs::search_by_title(String::from("shrek"))
///   .unwrap();
///
/// assert_eq!(shrek.Title, "Shrek");
/// ```
fn search_by_title(title: String) -> Result<Film, serde_json::Error>{
	let mut data = reqwest::get(
	  &format!("http://www.omdbapi.com/?apikey=21e783b3&t={}", title)[..]
	).unwrap();

	return match serde_json::from_str(&data.text().unwrap()){
		Ok(x) => Ok(x),
		Err(e) => Err(e)
	}
}

/// Searches for and returns a film in the OMDb.
///
/// Sends a request to OMDb for a film with the id `id` and returns a Film
/// object populated with the returned information. Does no input validation,
/// formatting, or case-switching, so can be temperamental. Returns a
/// `reqwest::Error` upon failure.
///
/// # Examples
///
/// To search for the film Shrek:
/// ``` ignore
/// use omdbrs;
///
/// let shrek: omdbrs::Film = omdbrs::search_by_id(String::from("tt0126029"))
///   .unwrap();
///
/// assert_eq!(shrek.Title, "Shrek");
/// ```
fn search_by_id(id: String) -> Result<Film, serde_json::Error>{
	let mut data = reqwest::get(
	  &format!("http://www.omdbapi.com/?apikey=21e783b3&i={}", id)[..]
	).unwrap();

	return match serde_json::from_str(&data.text().unwrap()){
		Ok(x) => Ok(x),
		Err(e) => Err(e)
	}
}
