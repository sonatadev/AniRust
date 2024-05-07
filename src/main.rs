

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::*;
use std::io;


#[derive(Debug, Deserialize)]
struct AnimeData {
    title : String,
    ongoing : bool,
    release_date : AnimeReleaseDate,
    nsfw : bool
}

#[derive(Debug, Deserialize)]
struct AnimeReleaseDate {
    year : i32,
    month : String
}

fn main(){
    
    let file = File::open("src/data.json").expect("Failed to open file");
    
    let reader = BufReader::new(file);
    
    let anime_catalogue : Vec<AnimeData> = serde_json::from_reader(reader).expect("Failed to parse json");

    println!("What is the name of the anime you want to search information about?");

    let mut anime_search = String::new();

    io::stdin().read_line(&mut anime_search)
        .expect("Failed to read line"); 

    println!();

    for anime in anime_catalogue {
        if(anime.title.trim().to_lowercase() == anime_search.trim().to_lowercase()){
            println!("Title: {}", anime.title);
            println!("Release Date: {} {}", anime.release_date.month, anime.release_date.year);
            println!("Ongoing: {}", anime.ongoing);
            println!("NSFW: {}", anime.nsfw);
            println!();
            main();
        }
    }
        println!("{} has not been found in our database. \n", anime_search.to_lowercase());
        main();
}

