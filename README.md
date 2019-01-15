# Motivate [![Build Status](https://travis-ci.com/ragreener1/Motivate.svg?branch=master)](https://travis-ci.com/ragreener1/Motivate)

This is my final year project for BSc Computer Science
## Setup
Install rust using [rustup](https://rustup.rs)

## Running Motivate
### Configuration
#### config/parameters.yaml
```yaml
---
total_years: the number of years the simulation runs for
number_of_people: the number of people in the simulation
number_of_simulations: the number of simulations to run in parallel
social_connectivity: how connected an agent is to its social network
subculture_connectivity: how connected an agent is to its subculture
neighbourhood_connectivity: how connected an agent is to its neighbourhood
number_of_social_network_links: the minimum number of links an agent should have in its social network
number_of_neighbour_links: the minimum number of neighbours an agent should be influenced by
days_in_habit_average: the number of days that account for approximately 86% of the habit average
distributions: this should not be changed
```
#### config/scenario.yaml
```yaml
---
id: the name of the scenario
subcultures: <- these are the subcultures in the scenario, currently an equal amount of agents in each
  - id: Subculture A <- the name of the subculture
    desirability: <- the desirability scores of transport modes in the subculture from 0 - 1
      PublicTransport: 0.5
      Walk: 0.699999988079071
      Car: 0.800000011920929
      Cycle: 0.8999999761581421
  < other subcultures omitted >
neighbourhoods:
  - id: "0" <- the name of the subulture
    supportiveness: <- how supportive the neighbourhood is for a transport mode
      Cycle: 0.699999988079071
      Walk: 0.800000011920929
      Car: 0.8999999761581421
      PublicTransport: 0.8999999761581421
    capacity: <- the maximum capacity at which there is no congestion penalty
      Car: 3600
      Cycle: 150000
      Walk: 150000
      PublicTransport: 3000
  < other neighbourhoods omitted >
number_of_bikes: 10000 <- How many bikes are in the scenario
number_of_cars: 5000 <- How many cars are in the scenario
intervention: <- The intervention that should occur
  day: 365 <- The day at which the intervention takes place
  neighbourhood_changes:
    - id: "0" <- The ID of the neighbourhood to change
      increase_in_supportiveness: <- How to change the supportiveness, be careful that this does not make the supportiveness < 0 or > 1
        Car: -0.1
        Cycle: 0.1
        < fields with no change are not required >
      increase_in_capacity: <- Be careful this does not make the capacity < 0
        Car: -4000
        PublicTransport: -100
      < if there is no increase_in_supportiveness or increase_in_capacity the respective field can be left out e.g. >
    - id: "1"
      increase_in_capacity:
        Car: -100
    - id: "2"
      increase_in_supportiveness:
        Walk: 0.4
        Cycle: 0.3
    < other changes to the neighbourhood can be added in the same way >
  subculture_changes: 
    - id: Subculture A <- The ID of the Subculture to change
      increase_in_desirability: <- How to change the desirability, be careful that this does not make the desirability < 0 or > 1
        Car: -0.1
        Cycle: 0.1
        < fields with no change are not required >
    < other changes to the subculture can be added in the same way >
  change_in_number_of_bikes: 10000 <- An increase (or decrease) in the number of bikes
  change_in_number_of_cars: -100 <- An increase (or decrease) in the number of cars
```

### Running the simulation

On the first run of the simulation run `cargo run --release -- --generate`,
so that social networks are generated.

Afterwards in the root of the repository run `cargo run --release`.

## Generating Documentation
Documentation can be generated by running `cargo rustdoc -- --document-private-items`
## Notes
As specified in .cargo/config the generated executable will target the cpu it is compiled on,
therefore binaries created on one system, may not execute on other systems, even if they run
the same operating system. To remove this requirement remove `-C target_cpu=native` from .cargo/config,
however this could have an impact on performance.
