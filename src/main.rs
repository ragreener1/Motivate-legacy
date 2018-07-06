extern crate itertools;
#[macro_use] extern crate maplit;
#[macro_use] extern crate log;
extern crate simple_logger;
extern crate serde_yaml;
extern crate im;
extern crate rand;
extern crate rayon;

mod weather;
mod transport_mode;
mod season;
mod journey_type;
mod neighbourhood;
mod subculture;
mod scenario;
mod agent;
mod simulation;
mod union_with;
mod social_network;
mod statistics;

use std::fs::File;
use std::collections::HashMap;
use std::time::SystemTime;
use std::sync::Arc;
use std::env;
use std::io::Write;
use std::io::prelude::*;
use rayon::prelude::*;
use weather::Weather;
use transport_mode::TransportMode;
use season::season;
use neighbourhood::Neighbourhood;
use subculture::Subculture;
use scenario::Scenario;

/// This is the entry point for the application
fn main() {
    // Create a new logger for system output
    simple_logger::init().unwrap();

    // Used for monitoring running time
    let t0 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Get the system arguments
    let args: Vec<String> = env::args().collect();

    ///////////////////////////////////////////////
    //           Model Parameters               ///
    ///////////////////////////////////////////////
    
    // Total number of years the simulation runs for
    let total_years = 1;
    // The number of people in the simulation
    let number_of_people = 30000;
    // The number of simulations that should take place per scenario
    let number_of_simulations_per_scenario = 4;
    // How connected an agent is to their social network
    let social_connectivity = 0.7f32;
    // How connected an agent is to their subculture
    let subculture_connectivity = 0.5f32;
    // How connected an agent is to their neighbourhood
    let neighbourhood_connectivity = 0.3f32;
    // The minimum number of links in their social network, and agent should have.
    // This is the mean number of social network links / 2
    let number_of_social_network_links = 10;
    // The minimum number of links in the neighbourhood-wide social network, an agent should have
    // This is the mean number of links / 2
    let number_of_neighbour_links = 10;
    // This is used as a weighting for the habit average, the most recent n days, account
    // for approximately 86% of the average
    let days_in_habit_average = 30;

    // If the generate flag is used
    if args.len() >= 2 {
        if &args[1] == "--generate" {
            // Generate as many social networks as number of simulations per scenario
            let numbers: Vec<u32> = (0..number_of_simulations_per_scenario).collect();
            // Get the networks stored as a YAML file
            let networks: Vec<String> = numbers
                .par_iter()
                .map(|_| serde_yaml::to_string(&social_network::generate_social_network(
                    number_of_social_network_links, number_of_people)).unwrap())
                .collect();

            // Create a networks directory to store them in
            std::fs::create_dir_all("networks")
                .expect("Failed to create networks directory");

            // For each network, save the network to a file
            networks
                .par_iter()
                .enumerate()
                .for_each(|(i, item)| {
                    let mut file = std::fs::File::create(format!("networks/{}.yaml", i+1)).ok().unwrap();
                    file.write_all(item.as_bytes()).ok();
                });
            info!("Generating networks complete")
        }
    }

    
    ///////////////////////////////////////////////
    //            Model Scenarios               ///
    ///////////////////////////////////////////////
    let scenarios: Vec<Scenario> = vec![
        Scenario {
            id: String::from("pre intervention"),
            subcultures: vec![
                Arc::new(Subculture {
                    id: "Subculture A".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    id: "Subculture B".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    id: "Subculture C".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Arc::new(Neighbourhood{
                    id: "0".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "1".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "2".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.2f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "3".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.2f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
            ),
        },
        Scenario {
            id: String::from("post intervention"),
            subcultures: vec![
                Arc::new(Subculture {
                    id: "Subculture A".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.8f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    id: "Subculture B".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::PublicTransport => 0.8f32,
                        TransportMode::Cycle => 0.6f32,
                        TransportMode::Walk => 0.7f32
                    }
                }),
                Arc::new(Subculture {
                    id: "Subculture C".to_owned(),
                    desirability: hashmap!{
                        TransportMode::Car => 0.4f32,
                        TransportMode::PublicTransport => 0.5f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32
                    }
                })
            ],
            neighbourhoods: vec!(
                Arc::new(Neighbourhood{
                    id: "0".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.9f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "1".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.5f32,
                        TransportMode::Cycle => 0.7f32,
                        TransportMode::Walk => 0.8f32,
                        TransportMode::PublicTransport => 1.0f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "4".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.7f32,
                        TransportMode::Cycle => 0.8f32,
                        TransportMode::Walk => 0.6f32,
                        TransportMode::PublicTransport => 0.5f32
                    }
                }),
                Arc::new(Neighbourhood{
                    id: "3".to_owned(),
                    supportiveness: hashmap!{
                        TransportMode::Car => 0.2f32,
                        TransportMode::Cycle => 0.9f32,
                        TransportMode::Walk => 0.9f32,
                        TransportMode::PublicTransport => 0.9f32
                    }
                }),
            ),
        }
    ];

    // Create a random weather pattern drawing from the percentage_bad_weather of each season
    let mut weather_pattern: HashMap<u32, Weather> = HashMap::new();

    for i in 0..(total_years * 365) {
        let current_season = season(i);
        let random_float = rand::random::<f32>();
        if random_float > current_season.percentage_bad_weather() {
            weather_pattern.insert(i, Weather::Good);
        } else {
            weather_pattern.insert(i, Weather::Bad);
        }
    }

    // Store a list of tuples of scenarios, and thread IDs
    let mut scenario_and_thread_ids: Vec<(&Scenario, String)> = Vec::new();

    for scenario in scenarios.iter() {
        for i in 1..=number_of_simulations_per_scenario {
            scenario_and_thread_ids.push((scenario,
                String::from("".to_owned() + &scenario.id[..] + "-" + &i.to_string())));
        }
    }

    // Run in parallel the simulations
    scenario_and_thread_ids.par_iter().for_each(|(scenario, thread_id)| {
        // Get the network number and load the network
        let network_number: String = thread_id
            .split("-")
            .last()
            .unwrap()
            .to_owned();
        let file = File::open(format!("networks/{}.yaml", network_number))
            .expect("File cannot be opened");

        let network = read_network(file);

        simulation::run(thread_id.to_string(),
                     scenario,
                     total_years,
                     number_of_people,
                     social_connectivity,
                     subculture_connectivity,
                     neighbourhood_connectivity,
                     number_of_neighbour_links,
                     days_in_habit_average,
                     &weather_pattern,
                     network)
                     .unwrap();
    });

    // Output the running time

    let t1 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    info!("TOTAL RUNNING TIME: {}s", t1 - t0)
}

/// Read a social network from a file
/// file: An input file in YAML mapping ids to a list of ids
/// Returns: A HashMap mapping ids, to the ids of their friends
fn read_network(mut file: File) -> HashMap<u32, Vec<u32>> {
    info!("READING NETWORK");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("There was an error reading the file");

    serde_yaml::from_slice(file_contents.as_bytes())
        .expect("There was an error parsing the file")
}
