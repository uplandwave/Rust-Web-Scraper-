// extern crate reqwest;
// extern crate scraper;

// use scraper::{Html, Selector};

// fn main(){
//     println!("Here is the data: ");
//     scrape_team_data("https://www.espn.com/nba/team/_/name/utah/utah-jazz");
// }

// fn scrape_team_data(url:&str){

//     let mut req = reqwest::blocking::get(url).unwrap();
//     assert!(req.status().is_success());
//     let doc_body = Html::parse_document(&req.text().unwrap());

//     // println!("{:?}",req)

//     let team = Selector::parse(".db .pr3 .nowrap").unwrap();

//     for team in doc_body.select(&team){
//         let teams = team.text().collect::<Vec<_>>();
//         println!("{}", teams[0]);
//     }

// }

use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    // The URL of the website you want to scrape
    let url = "https://www.imdb.com/chart/top/?ref_=nv_mv_250";

    // Fetch the HTML content of the website
    let body = match get(url) {
        Ok(response) => response.text().unwrap(),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    // Creating a list to store values in
    let mut list_list: Vec<String> = Vec::new();
    let mut year_list: Vec<String> = Vec::new();
    let mut rating_list: Vec<String> = Vec::new();
    let mut duration_list: Vec<String> = Vec::new();

    // sets up i values
    let i_values = vec![1, 2, 3];


    // Parse the HTML content with the scraper library
    let document = Html::parse_document(&body);

    // Define the CSS selector to pick a class
    let selector = Selector::parse(".sc-1e00898e-8.hsHAHC.cli-title-metadata-item").unwrap();

    // Extract elements matching the selector
    let matches: Vec<_> = document.select(&selector).collect();

    // Print the text content of each matching element
    for (index, element) in document.select(&selector).enumerate() {
        // println!("{}", element.text().collect::<String>());
        // let addNumber = format!("{}" "{}", i, element.text().collect::<String>());
        // year_list.push(element.text().collect::<String>());
        // let i = 1;
        let i = i_values[index % i_values.len()];
        let bepbup = format!("{} {}", i, element.text().collect::<String>());
        list_list.push(bepbup.clone());

        // this is where is separate the items into different lists
        match bepbup.chars().next() {
            Some('1') => rating_list.push(bepbup),
            Some('2') => year_list.push(bepbup),
            Some('3') => duration_list.push(bepbup),
            _ => (),
        }
    }

    for i in year_list {
        println!("{}", i)
    }    
    for i in rating_list {
        println!("{}", i)
    }    
    for i in duration_list {
        println!("{}", i)
    }
        // here i am working on moving the lists onto there own csv files
//     let mut html_file = File::create("html_output.csv")?;
//     write!(html_file, "{}", body)?;
}

// use reqwest::blocking::get;
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use csv::Writer;

// fn main() {
//     if let Err(e) = run() {
//         eprintln!("Error: {}", e);
//     }
// }

// fn run() -> Result<(), Box<dyn Error>> {
//     // The URL of the website you want to scrape
//     let url = "https://www.tiktok.com/explore";

//     // Fetch the HTML content of the website
//     let body = match get(url) {
//         Ok(response) => response.text()?,
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return Err(e.into());
//         }
//     };

//     // Print the HTML content
//     // println!("{}", body);

//     let mut html_file = File::create("html_output.csv")?;
//     write!(html_file, "{}", body)?;

//     Ok(())
// }


// run the base but to check
// use reqwest::blocking::get;
// use scraper::{Html, Selector};
// use csv::Writer;
// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     // The URL of the website you want to scrape
//     let url = "https://www.tiktok.com/explore";

//     // Fetch the HTML content of the website
//     let body = match get(url) {
//         Ok(response) => response.text().unwrap(),
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return;
//         }
//     };

//     // println!("{}", body);

//     let mut html_file = File::create("html_output.csv")?;
//     write!(html_file, "{}", body)?;

    // // Parse the HTML content with the scraper library
    // let document = Html::parse_document(&body);

    // // Define the CSS selector for the class "db pr3 nowrap"
    // let selector = Selector::parse(".css-1as5cen-DivWrapper.e1cg0wnj1").unwrap();

    // // Extract elements matching the selector
    // let matches: Vec<_> = document.select(&selector).collect();

    // // Print the text content of each matching element
    // for element in matches {
    //     println!("{}", element.text().collect::<String>());
    // }
//}



// run with csv file

// use reqwest::blocking::get;
// use scraper::{Html, Selector};
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use csv::Writer;

// fn main() -> Result<(), Box<dyn Error>> {
//     // The URL of the website you want to scrape
//     let url = "https://www.tiktok.com/explore";

//     // Fetch the HTML content of the website
//     let body = match get(url) {
//         Ok(response) => response.text().unwrap(),
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return Err(e.into());
//         }
//     };

//     // Parse the HTML content with the scraper library
//     let document = Html::parse_document(&body);

//     // Define the correct CSS selectors for the information you want to extract
//     let video_views_selector = Selector::parse(".css-fjta3j-StrongVideoCount.e19c29qe20").unwrap();
//     let likes_selector = Selector::parse(".css-8q3c8f-StrongVideoCount.ewsiauk7").unwrap();
//     // Add more selectors for other pieces of information as needed

//     // Extract elements matching the selectors
//     let video_views: Vec<_> = document.select(&video_views_selector).collect();
//     let likes: Vec<_> = document.select(&likes_selector).collect();
//     // Extract more information using additional selectors

//     // Open a CSV file for writing
//     let mut csv_writer = Writer::from_path("output.csv")?;

//     // Write the header row to the CSV file
//     csv_writer.write_record(&["Video Views", "Likes"])?;

//     // Iterate through the extracted data and write it to the CSV file
//     for (views, like) in video_views.iter().zip(likes.iter()) {
//         let views_text = views.text().collect::<String>();
//         let likes_text = like.text().collect::<String>();
//         // Extract more data as needed

//         // Write a new row to the CSV file
//         csv_writer.write_record(&[views_text, likes_text])?;
//     }

//     // Flush the CSV writer to ensure all data is written to the file
//     csv_writer.flush()?;

//     Ok(())
// }
