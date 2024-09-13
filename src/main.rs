use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::{DateTime, Local, Utc};

struct Info {
    name: String, 
    email: String, 
    role: String, 
    years_employed: f32,
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        let _ = write!(f, "- {} has been at CARFAX for {} years. They are a {} on the DS Data Science Team! 😄😄 If you need them, their email is {}.", 
                       self.name, self.years_employed, self.role, self.email);
        
        Ok(())
    }
}

enum DsTeam {
    Tiger, 
    Wing, 
    Denis, 
    Himalaya, 
    Luna, 
    Jack, 
    Lakshman, 
    Gloria, 
    Kyuson, 
    Xinyu,  
    Paul, 
    Cory,
}

fn main() {
    let mut cwkr = vec![];

    let today: DateTime<Local> = Local::now();
    let today_clean = today.format("%Y-%m-%d");

    cwkr.push(Info{name: String::from("Tiger Tang"), email: "tigertang@carfax.com".to_string(), role: String::from("Senior Manager"), years_employed: 9.0});
    cwkr.push(Info{name: String::from("Wing Tian"), email: String::from("wingtian@carfax.com"), role: String::from("Senior Data Scientist"), years_employed: 1.0});
    cwkr.push(Info{name: String::from("Denis Voronov"), email: String::from("denisvoronov@carfax.com"), role: String::from("Denior Data Analyst"), years_employed: 2.5});
    cwkr.push(Info{name: String::from("Himalaya Sharma"), email: String::from("himalayasharma@carfax.com"), role: String::from("Data Scientist"), years_employed: 1.0});
    cwkr.push(Info{name: String::from("Luna Xia"), email: String::from("lunaxia@carfax.com"), role: String::from("Data Scientist"), years_employed: 0.5});
    cwkr.push(Info{name: String::from("Jack Blumstein"), email: String::from("johnblumstein@carfax.com"), role: String::from("Associate Data Scientist"), years_employed: 1.0});
    cwkr.push(Info{name: String::from("Lakshman Balaji"), email: String::from("lakshmanbalaji@carfax.com"), role: String::from("Senior Data Analyst"), years_employed: 0.1});
    cwkr.push(Info{name: String::from("Gloria Liu"), email: String::from("glorialiu@carfax.com"), role: String::from("Data Scientist"), years_employed: 2.75});
    cwkr.push(Info{name: String::from("Kyuson Lim"), email: String::from("kyusonlim@carfax.com"), role: String::from("Co-Op"), years_employed: 0.1});
    cwkr.push(Info{name: String::from("Xinyu Gao"), email: String::from("xinyugao@carfax.com"), role: String::from("Senior Data Analyst"), years_employed: 5.0});
    cwkr.push(Info{name: String::from("Paul Ajayi"), email: String::from("paulajayi@carfax.com"), role: String::from("Data Analyst"), years_employed: 3.5});
    cwkr.push(Info{name: String::from("Cory Pilat"), email: String::from("corypilat@carfax.com"), role: String::from("Senior Data Analyst, Team Lead"), years_employed: 2.5});

    println!("({}) The current state of the DS team is: \n", today_clean);

    for i in &cwkr {
        println!("{i}")
    }



}
