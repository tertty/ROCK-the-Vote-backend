#[macro_use] extern crate rocket;

use std::sync::Mutex;

use rocket::State;
use rocket::http::Status;
use rocket::serde::{Serialize, json::Json};

use rusqlite::{Connection, Result, params};

use chrono::{Datelike, Utc};

/// Enum that represents the type of daily question being asked.
/// 
/// Can be:
/// Would You Rather
/// Who Would Win
/// This Or That
#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
enum QuestionType {
    WYR,
    WWW,
    TOT
}

/// Struct that represents JSON payload sent to RTV Pebble client.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct QuestionResultsJSONResponse {
    red_prompt: String,             // Current/Previous day's red prompt. 
    blue_prompt: String,            // Current/Previous day's blue prompt.
    question_type: QuestionType,    // Current/Previous day's QuestionType.
    red_count: u64,                 // Current/Previous day's red prompt vote count.
    blue_count: u64                 // Current/Previous day's blue prompt vote count.
}

/// Struct that represents data that needs to be shared with all db related functions.
struct PersistentData {
    db_conn: Connection,                                    // Open connection on sqlite database opened at new PersistentData.
    rtv_prompts: [Vec<(String, String, QuestionType)>; 12], // RTV year prompts initialized at new PersistentData.
    current_month: u32,                                     // Current month of the year's number.
    current_day: u32                                        // Current day of the month's number.
}

impl PersistentData {
    /// Create a new instance of the mysqlite database file and initialize new database.
    /// Database is generated on every launch, we do not current support picking up from where you left off.
    fn new() -> Result<Self> {
        let db_conn = Connection::open("wyr_persistent.db")?;
    
        db_conn.execute(
            "CREATE TABLE IF NOT EXISTS vote_count (
                question_number INT PRIMARY KEY,
                red_vote_count INT,
                blue_vote_count INT
            )",
            (),
        )?;
        
        // Vec of a (String, String, QuestionType) tuple that represents a full year of prompts.
        // Each index is a month (0..11), and each index is a vec of the tuple that represents the days of the month (0..x).
        let rtv_prompts = [
            // Jan
            vec![

            ],
            // Feb
            vec![

            ],
            // March
            vec![
                ("Move like a robot".to_string(), "Talk like a robot".to_string(), QuestionType::WYR),
                ("Chocolate".to_string(), "Vanilla".to_string(), QuestionType::TOT),
                ("Thor".to_string(), "The Hulk".to_string(), QuestionType::WWW),
                ("Mario".to_string(), "Sonic".to_string(), QuestionType::WWW),
                ("Roomba with a knife".to_string(), "Drone with a water gun".to_string(), QuestionType::WWW),
                ("Have hair that changes color based on mood".to_string(), "Have glow in the dark skin".to_string(), QuestionType::WYR),
                ("Star Wars".to_string(), "Star Trek".to_string(), QuestionType::TOT),
                ("Coffee".to_string(), "Tea".to_string(), QuestionType::TOT),
                ("Live without music".to_string(), "Live without TV".to_string(), QuestionType::WYR),
                ("Link".to_string(), "Kratos".to_string(), QuestionType::WWW),
                ("Tesla".to_string(), "Edison".to_string(), QuestionType::WWW),
                ("Terraria".to_string(), "Minecraft".to_string(), QuestionType::TOT),
                ("Spicy".to_string(), "Mild".to_string(), QuestionType::TOT),
                ("Only whisper".to_string(), "Only shout".to_string(), QuestionType::WYR),
                ("Tetris Blocks".to_string(), "Minecraft Blocks".to_string(), QuestionType::WWW),
                ("Only talk in riddles".to_string(), "Only talk in emojis".to_string(), QuestionType::WYR),
                ("Talk to plants".to_string(), "Talk to animals".to_string(), QuestionType::WYR),
                ("Bowser".to_string(), "Dr. Robotnik".to_string(), QuestionType::WWW),
                ("Hot Coffee".to_string(), "Iced Coffee".to_string(), QuestionType::TOT),
                ("Cake".to_string(), "Pie".to_string(), QuestionType::TOT),
                ("The Simpsons".to_string(), "Family Guy".to_string(), QuestionType::TOT),
                ("Iron Man".to_string(), "Captian American".to_string(), QuestionType::WWW),
                ("Godzilla".to_string(), "King Kong".to_string(), QuestionType::WWW),
                ("Pizza".to_string(), "Burgers".to_string(), QuestionType::TOT),
                ("Always burp confetti".to_string(), "Always sneeze glitter".to_string(), QuestionType::WYR),
                ("Ice Cream".to_string(), "Cake".to_string(), QuestionType::TOT),
                ("Have an extra eye".to_string(), "Have an extra ear".to_string(), QuestionType::WYR),
                ("Wear a tuxedo to bed".to_string(), "Wear pajamas to every formal event".to_string(), QuestionType::WYR),
                ("Have a mariachi band".to_string(), "Have a laugh track".to_string(), QuestionType::WYR),
                ("Always wear clown shoes".to_string(), "Always wear a giant sombrero".to_string(), QuestionType::WYR),
                ("A lion with eagle wings".to_string(), "A shark that can breathe air".to_string(), QuestionType::WWW),
            ],
            // April
            vec![
                ("Johnny English".to_string(),"Mr. Bean".to_string(), QuestionType::WWW),
                ("The Beach".to_string(),"The Mountains".to_string(), QuestionType::TOT),
                ("Be able to control fire".to_string(),"Be able to control water".to_string(), QuestionType::WYR),
                ("You, literally".to_string(),"A horde of five year olds".to_string(), QuestionType::WWW),
                ("iPhone".to_string(),"Android".to_string(), QuestionType::TOT),
                ("Go to the future".to_string(),"Go to the past".to_string(), QuestionType::WYR),
                ("One Trillion Lions".to_string(),"The Sun".to_string(), QuestionType::WWW),
                ("Never use social media again".to_string(),"Never watch a movie again".to_string(), QuestionType::WYR),
                ("Boba Fett".to_string(),"The Predator".to_string(), QuestionType::WWW),
                ("Peanut butter".to_string(),"Jelly".to_string(), QuestionType::TOT),
                ("Always be 10 minutes late".to_string(),"Always be 20 minutes early".to_string(), QuestionType::WYR),
                ("The Kool-Aid Man".to_string(),"The Michelin Man".to_string(), QuestionType::WWW),
                ("Halloween".to_string(),"Christmas".to_string(), QuestionType::TOT),
                ("Go to the Moon".to_string(),"Go to Mars".to_string(), QuestionType::WYR),
                ("Shark with bear hands".to_string(),"Bear with shark hands".to_string(), QuestionType::WWW),
                ("PC".to_string(),"Console".to_string(), QuestionType::TOT),
                ("Lose the ability to read".to_string(),"Lose the ability to speak".to_string(), QuestionType::WYR),
                ("Waffles".to_string(),"Pancakes".to_string(), QuestionType::TOT),
                ("Gandalf".to_string(),"Dumbledore".to_string(), QuestionType::WWW),
                ("The city".to_string(),"The country".to_string(), QuestionType::TOT),
                ("Be a famous director".to_string(),"Be a famous actor".to_string(), QuestionType::WYR),
                ("Ned Flanders".to_string(),"Mr. Rogers".to_string(), QuestionType::WWW),
                ("Have a bottomless box of Legos".to_string(),"Have a bottomless gas tank".to_string(), QuestionType::WYR),
                ("Open gifts on Christmas Eve".to_string(),"Open gifts on Christmas Day".to_string(), QuestionType::WYR),
                ("Bath".to_string(),"Shower".to_string(), QuestionType::TOT),
                ("Spend the weekend with pirates".to_string(),"Spend the weekend with ninjas".to_string(), QuestionType::WYR),
                ("Drink sour milk".to_string(),"Brush your teeth with soap".to_string(), QuestionType::WYR),
                ("Grilled Cheese".to_string(),"Tacos".to_string(), QuestionType::TOT),
                ("Goku".to_string(),"Superman".to_string(), QuestionType::WWW),
                ("Master Chief".to_string(),"The Doom Slayer".to_string(), QuestionType::WWW)
            ],
            // May
            vec![
                ("Star in a TV show".to_string(),"Star in a movie".to_string(), QuestionType::WYR),
                ("Big city".to_string(), "Small town".to_string(), QuestionType::TOT),
                ("Spring".to_string(), "Fall".to_string(), QuestionType::TOT),
                ("Be beautiful and stupid".to_string(),"Be unattractive but a genius".to_string(), QuestionType::WYR),
                ("Coke".to_string(),"Pepsi".to_string(), QuestionType::TOT),
                ("Day".to_string(), "Night".to_string(), QuestionType::TOT),
                ("Neo".to_string(), "John Wick".to_string(), QuestionType::WWW),
                ("Cold".to_string(), "Hot".to_string(), QuestionType::TOT),
                ("Be able to fly".to_string(),"Be able to teleport".to_string(), QuestionType::WYR),
                ("R2-D2".to_string(), "WALL-E".to_string(), QuestionType::WWW),
                ("Dogs".to_string(),"Cats".to_string(), QuestionType::TOT),
                ("James Bond".to_string(),"Jason Bourne".to_string(), QuestionType::WWW),
                ("The Flash".to_string(), "Quicksilver".to_string(), QuestionType::WWW),
                ("Batman".to_string(),"Superman".to_string(), QuestionType::WWW),
                ("Summer".to_string(), "Winter".to_string(), QuestionType::TOT),
                ("Have seven fingers on each hand".to_string(),"Have seven toes on each foot".to_string(), QuestionType::WYR),
                ("Red".to_string(),"Blue".to_string(), QuestionType::TOT),
                ("Team Fortress 2".to_string(), "Overwatch".to_string(), QuestionType::TOT),
                ("Xbox".to_string(),"Playstation".to_string(), QuestionType::TOT),
                ("Dine in".to_string(),"Eat out".to_string(), QuestionType::TOT),
                ("Rain".to_string(), "Snow".to_string(), QuestionType::TOT),
                ("Move like a robot".to_string(), "Talk like a robot".to_string(), QuestionType::WYR),
                ("George Costanza".to_string(),"Jerry Seinfeld".to_string(), QuestionType::WWW),
                ("Robocop".to_string(),"The Terminator (T-800)".to_string(), QuestionType::WWW),
                ("Have a photographic memory".to_string(),"Be a genius".to_string(), QuestionType::WYR),
                ("Yoda".to_string(), "Dumbledore".to_string(), QuestionType::WWW),
                ("Hot weather".to_string(),"Cold weather".to_string(), QuestionType::TOT),
                ("Have a pet dragon".to_string(), "Have a pet unicorn".to_string(), QuestionType::WYR),
                ("live in a treehouse".to_string(), "live in a cave".to_string(), QuestionType::WYR),
                ("Optimus Prime".to_string(), "Voltron".to_string(), QuestionType::WWW),
                ("Summer".to_string(), "Winter".to_string(), QuestionType::TOT),        
            ],
            // June TBD
            vec![
                ("Comedy".to_string(), "Horror".to_string(), QuestionType::TOT),
                ("Books".to_string(), "Movies".to_string(), QuestionType::TOT),
                ("Zombies".to_string(), "Vampires".to_string(), QuestionType::TOT),
                ("Aliens".to_string(), "Ghosts".to_string(), QuestionType::TOT),
                ("Avatar".to_string(), "Dune".to_string(), QuestionType::TOT),
                ("Pokémon".to_string(), "Digimon".to_string(), QuestionType::TOT),
                ("Tintin".to_string(), "Indiana Jones".to_string(), QuestionType::TOT),
                ("Nintendo".to_string(), "Sega".to_string(), QuestionType::TOT),
                ("Smash Bros".to_string(), "Tekken".to_string(), QuestionType::TOT),
                ("Street Fighter".to_string(), "Mortal Kombat".to_string(), QuestionType::TOT),
                ("Guitar Hero".to_string(), "Dance Dance Revolution".to_string(), QuestionType::TOT),
                ("Sneakers".to_string(), "Sandals".to_string(), QuestionType::TOT),
                ("Homer Simpson".to_string(), "Peter Griffin".to_string(), QuestionType::WWW),
            ],
            // July
            vec![

            ],
            // August
            vec![

            ],            
            // September
            vec![],
            // October
            vec![
                ("Freddy Krueger".to_string(), "Pennywise".to_string(), QuestionType::WWW),
                ("Dracula".to_string(), "Frankenstein".to_string(), QuestionType::WWW),
                ("Jason Voorhees".to_string(), "Michael Myers".to_string(), QuestionType::WWW),
                ("The Mummy".to_string(), "The Werewolf".to_string(), QuestionType::WWW),
                ("Ghostface".to_string(), "Jigsaw".to_string(), QuestionType::WWW),
                ("The Demogorgon".to_string(), "The Xenomorph".to_string(), QuestionType::WWW),
            ],
            // November
            vec![

            ],
            // December
            vec![

            ]
        ];
    
        Ok(
            PersistentData { 
                db_conn,
                rtv_prompts,
                current_month: 0,
                current_day: 0
            }
        )
    }

    /// For every API call done by user, check if it's been a full day since the last API call.
    /// 
    /// This is performed "on demand" so we don't have to keep a timer/loop checking to see if a new day has begun.
    /// The operations for a new day are very light so there won't be "lag" on the user who kicks off the new day.
    /// After all, if no one is using the API does it even exist? 
    fn has_a_new_day_begun(&mut self) -> Result<()> {
        let current_date = Utc::now();

        // If it's a new day...
        // Create new day's result table.
        // Drop blacklist of previous day's responders.
        if (current_date.day() != self.current_day) {
            self.db_conn.execute(
                "INSERT INTO vote_count (question_number, red_vote_count, blue_vote_count) VALUES (?1, 0, 0)", 
            params!(&current_date.day()))?;

            self.db_conn.execute(
                "DROP TABLE IF EXISTS responders",
                ()
            )?;
        
            self.db_conn.execute(
                "CREATE TABLE responders (
                    voter_id CHARACTER(16) PRIMARY KEY,
                    response BOOLEAN NOT NULL
                )",
                (),
            )?;

            self.current_day = current_date.day();
            self.current_month = current_date.month();
        }

        Ok(())
    }

    /// Grab day's prompt by index from rtv_prompts vec.
    fn get_latest_prompts(&self) -> (String, String, QuestionType) {
        return (self.rtv_prompts[(self.current_month - 1) as usize][(self.current_day - 1) as usize].clone());
    }

    /// Grab previous day's prompt by index of current day minus one from rtv_prompts vec.
    fn get_previous_prompts(&self) -> (String, String, QuestionType) {
        // Check if we're on the first day of the month, because we don't want to underflow!
        if (self.current_day == 1) {
            return self.rtv_prompts[(self.current_month - 1) as usize][29].clone();
        } else {
            return self.rtv_prompts[(self.current_month - 1) as usize][((self.current_day - 1) - 1) as usize].clone();
        }
    }

    /// Verify user calling RTV API has not previously voted.
    /// 
    /// This is done by taking the Pebble client's UUID and seeing if it exists in our responder's blacklist table.
    fn has_user_voted(&mut self, voter_uuid: &String) -> Result<bool> {
        self.has_a_new_day_begun()?;

        let mut does_uuid_exist_query_statement = self.db_conn.prepare("SELECT * FROM responders WHERE voter_id = ?1")?;

        return does_uuid_exist_query_statement.exists([voter_uuid])
    }
    
    /// Grab a user's vote and increment the count for that choice. Add voted user to blacklist table so they can't vote again.
    fn db_increment(&mut self, voter_uuid: String, which_increment: bool) -> Result<()> {    
        self.has_a_new_day_begun()?;

        // Check if user has not previously voted...
        if !self.has_user_voted(&voter_uuid)? {
            let mut get_current_vote_count_query_statement =  self.db_conn.prepare("SELECT red_vote_count, blue_vote_count FROM vote_count WHERE question_number = ?1")?;

            get_current_vote_count_query_statement.query_row([self.current_day], |row|{
                let current_red_vote_count: u64 = row.get(0)?;
                let current_blue_vote_count: u64 = row.get(1)?;
    
                // true increment red, false increment blue
                if (which_increment) {
                    self.db_conn.execute(
                        "UPDATE vote_count SET red_vote_count=?1 WHERE question_number=?2",
                        (current_red_vote_count + 1, self.current_day),
                    )?;
                } else {
                    self.db_conn.execute(
                        "UPDATE vote_count SET blue_vote_count=?1 WHERE question_number=?2",
                        (current_blue_vote_count + 1, self.current_day),
                    )?;
                }

                self.db_conn.execute(
                    "INSERT INTO responders (voter_id, response) VALUES (?1, ?2)",
                    params!(voter_uuid, which_increment),
                )?;
    
                Ok(())
            })?;
        
            return Ok(())
        } else {
            return Err(rusqlite::Error::ExecuteReturnedResults)
        }

    }
    
    /// Grab latest count for the current day's choices.
    fn db_latest_count(&mut self) -> Result<(u64, u64)> {
        self.has_a_new_day_begun()?;

        let mut query_statement =  self.db_conn.prepare("SELECT red_vote_count, blue_vote_count FROM vote_count WHERE question_number=?")?;
    
        let query_result = query_statement.query_row([self.current_day], |row|{
            Ok((row.get(0)?, row.get(1)?))
        })?;

        Ok(query_result)
    }

    /// Grab latest count for the previous day's choices.
    fn db_previous_count(&mut self) -> Result<(u64, u64)> {
        self.has_a_new_day_begun()?;

        let mut query_statement =  self.db_conn.prepare("SELECT red_vote_count, blue_vote_count FROM vote_count WHERE question_number=?")?;
    
        let query_result = query_statement.query_row([(if (self.current_day == 1){30}else{self.current_day} - 1) as usize], |row|{
            Ok((row.get(0)?, row.get(1)?))
        })?;

        Ok(query_result)
    }
}

/// API endpoint for POST-ing vote for red choice.
#[post("/increment_red/<voter_uuid>")]
fn post_increment_red(persistent_data: &State<Mutex<PersistentData>>, voter_uuid: String) -> Status {
    return match persistent_data.lock().unwrap().db_increment(voter_uuid, true) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

/// API endpoint for POST-ing vote for blue choice.
#[post("/increment_blue/<voter_uuid>")]
fn post_increment_blue(persistent_data: &State<Mutex<PersistentData>>, voter_uuid: String) -> Status {
    return match persistent_data.lock().unwrap().db_increment(voter_uuid, false) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

/// API endpoint for GET-ing if user has already voted for current day's prompt.
#[get("/has_user_voted/<voter_uuid>")]
fn has_user_voted(persistent_data: &State<Mutex<PersistentData>>, voter_uuid: String) -> String {
    return persistent_data.lock().unwrap().has_user_voted(&voter_uuid).unwrap().to_string();
}

/// API endpoint for GET-ing current day's prompt and poll results.
#[get("/latest_question_and_results")]
fn get_latest_question_and_results(persistent_data: &State<Mutex<PersistentData>>) -> Json<QuestionResultsJSONResponse> {
    let mut persistent_data = persistent_data.lock().unwrap();

    let (red_count, blue_count) = match persistent_data.db_latest_count() {
        Ok((red_count, blue_count)) => (red_count, blue_count),
        Err(_) => (0,0)
    };

    let (latest_red_prompt, latest_blue_prompt, question_type) = persistent_data.get_latest_prompts();

    Json (
        QuestionResultsJSONResponse { 
            red_prompt: latest_red_prompt.to_string(),
            blue_prompt: latest_blue_prompt.to_string(),
            question_type,
            red_count,
            blue_count
        }
    )
}

/// API endpoint for GET-ing previous day's prompt and poll results.
#[get("/previous_question_and_results")]
fn get_previous_question_and_results(persistent_data: &State<Mutex<PersistentData>>) -> Json<QuestionResultsJSONResponse> {
    let mut persistent_data = persistent_data.lock().unwrap();

    let (red_count, blue_count) = match persistent_data.db_previous_count() {
        Ok((red_count, blue_count)) => (red_count, blue_count),
        Err(_) => (0,0)
    };

    let (latest_red_prompt, latest_blue_prompt, question_type) = persistent_data.get_previous_prompts();

    Json (
        QuestionResultsJSONResponse { 
            red_prompt: latest_red_prompt.to_string(),
            blue_prompt: latest_blue_prompt.to_string(),
            question_type,
            red_count,
            blue_count
        }
    )
}

/// Rocket "main" that initializes RTV API endpoints.
#[launch]
fn rocket() -> _ {
    match PersistentData::new() {
        Ok(persistent_data) => rocket::build()
                    .manage(Mutex::new(persistent_data))
                    .mount("/api/rtv/", routes![post_increment_red, post_increment_blue, get_latest_question_and_results, get_previous_question_and_results, has_user_voted]),
        Err(e) => panic!("{}", e)
    }
}