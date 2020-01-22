//! A library for interacting with the OMDb in Rust.
//!
//! Interactions are mostly through the `Film` object, which is constructed
//! using some information applicable to an OMDb search, and which can then be
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
        assert_eq!(search_by_title(String::from("shrek"),
                                   &String::from("21e783b3")).unwrap().Title,
                   "Shrek");
        assert_eq!(search_by_title(String::from("shrek"),
                                   &String::from("21e783b3")).unwrap().Year,
                   "2001");
    }

    #[test]
    fn search_by_id_test(){
        assert_eq!(search_by_id(String::from("tt0126029"),
                                String::from("21e783b3")).unwrap().Title,
                   "Shrek");
        assert_eq!(search_by_id(String::from("tt0126029"),
                                String::from("21e783b3")).unwrap().Year,
                   "2001");
    }

    #[test]
    fn from_title_test(){
        let film: Film = Film::from_title(String::from("Shrek"),
                                          String::from("21e783b3")).unwrap();

        assert_eq!(film.Title, "Shrek");
        assert_eq!(film.Year, "2001");
        assert_eq!(film.Runtime, "90 min");

        assert!(Film::from_title(String::from("gobbeldygookasdfblu"),
                                 String::from("asdf")).is_err());
    }

    #[test]
    fn from_id_test(){
        let film: Film = Film::from_id(String::from("tt0126029"),
                                       String::from("21e783b3")).unwrap();

        assert_eq!(film.Title, "Shrek");
        assert_eq!(film.Year, "2001");

        assert!(Film::from_id(String::from("gobbeldygookasdfblur"),
                              String::from("asdf")).is_err());
    }

    #[test]
    fn test_search_for(){
        println!("Testing search_for");
        let results: Vec<Film> = match Film::search_for(String::from("shrek"),
                                                        String::from("21e783b3")
                                                       ){
            Ok(x) => x,
            Err(e) => panic!("Fucked it!: {}", e)
        };

        println!("Got past search");
        assert_eq!(results[0].Title, "Shrek");
        assert_eq!(results[1].Title, "Shrek 2");
    }
}


use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use custom_error::custom_error;
use regex::Regex;


custom_error!{pub FilmError
    /// Denotes an inability to find a film in a search
    FilmNotFound = "No film matching the given criteria was found",

    /// Denotes a sufficient number of results in a search
    NotEnoughResults = "Not enough results matching that title were found",
}


/// Structure for holding information about a Film.
///
/// Stores information about a film as retrieved from the OMDb. Should be
/// constructed using either `from_title()` or `from_id()`. This information can
/// be accessed using the included getters. All information is stored as
/// Strings.
///
/// # Examples
///
/// To create a Film object representing the film Shrek:
/// ```
/// use omdbrs::Film;
/// 
/// let film: Film = Film::from_title(String::from("Shrek"),
///                                   String::from("21e783b3")).unwrap();
///
/// assert_eq!(film.get_title(), "Shrek");
/// ```
/// In practice, `unwrap()` should not be used as a `FilmError` may be returned.
#[derive(Debug, Serialize, Deserialize)]
pub struct Film{
    Title: String,
    Year: String,
    Runtime: String,
    Rated: String,
    Released: String,
    Genre: String,
    Director: String,
    Writer: String,
    Actors: String,
    Plot: String,
    Language: String
}

impl Film{
    /// Constructor for a Film object using a film's title.
    ///
    /// Creates a Film object using the result of an OMDb query using the given
    /// title. If no matching film is found, a `FilmError` is returned instead.
    pub fn from_title(title: String, key: String) -> Result<Film, FilmError>{
        let film: Film = match search_by_title(title, &key){
            Ok(x) => x,
            Err(e) => return Err(FilmError::FilmNotFound)
        };

        Ok(film)
    }

    /// Constructor for a Film object using a film's IMDB id.
    ///
    /// Creates a Film object using the result of an OMDb query using the given
    /// id. If no matching film is found, a `FilmError` is returned instead.
    pub fn from_id(id: String, key: String) -> Result<Film, FilmError>{
        let film: Film = match search_by_id(id, key){
            Ok(x) => x,
            Err(e) => return Err(FilmError::FilmNotFound)
        };

        Ok(film)
    }

    /// Returns a list of results found in a search for the given title.
    ///
    /// Pulls the titles from a list of results to the query `title` and returns
    /// a vector of `Film` objects containing each matching film in the OMDb.
    pub fn search_for(title: String,key: String) -> Result<Vec<Film>,FilmError>{
        let mut results: Vec<Film> = Vec::new();
        let formatter: Regex = Regex::new(r###""Title":"[\w\s]+?""###).unwrap();

        println!("Initialised regex");

        let mut data = reqwest::get(
          &format!("http://www.omdbapi.com/?apikey={}&s={}", key, title)[..]
        ).unwrap();
        println!("Got data");

        let text = data.text().unwrap();
        println!("Got text from data:\n{}\n", text);

        let captures = formatter.find_iter(&text);

        // println!("Got {}", captures.get(0).unwrap().as_str());
        // println!("Got {}", captures.get(1).unwrap().as_str());

        for i in captures{
            let x = &i.as_str().chars().count();
            let title = &i.as_str()[9..(*x - 1)];
            results.push(Film::from_title(String::from(title),
                                          String::from("21e783b3"))?);
        }

        Ok(results)
    }

    // ==========
    // Getters
    // ==========
    pub fn get_title(&self) -> String{ String::from(&self.Title) }

    pub fn get_year(&self) -> String{ String::from(&self.Year) }

    pub fn get_runtime(&self) -> String{ String::from(&self.Runtime) }

    pub fn get_rated(&self) -> String{ String::from(&self.Rated) }

    pub fn get_released(&self) -> String{ String::from(&self.Released) }

    pub fn get_genre(&self) -> String{ String::from(&self.Genre) }

    pub fn get_director(&self) -> String{ String::from(&self.Director) }

    pub fn get_writer(&self) -> String{ String::from(&self.Writer) }

    pub fn get_actors(&self) -> String{ String::from(&self.Actors) }

    pub fn get_plot(&self) -> String{ String::from(&self.Plot) }

    pub fn get_language(&self) -> String{ String::from(&self.Language) }
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
fn search_by_title(title:String, key:&String) -> Result<Film, serde_json::Error>{
    let mut data = reqwest::get(
      &format!("http://www.omdbapi.com/?apikey={}&t={}", key, title)[..]
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
fn search_by_id(id: String, key: String) -> Result<Film, serde_json::Error>{
    let mut data = reqwest::get(
      &format!("http://www.omdbapi.com/?apikey={}&i={}", key, id)[..]
    ).unwrap();

    return match serde_json::from_str(&data.text().unwrap()){
        Ok(x) => Ok(x),
        Err(e) => Err(e)
    }
}
