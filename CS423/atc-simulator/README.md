The Rust-y ATC is an interactive, terminal-based Air Traffic Control simulation game written in Rust. You play as an ATC controller managing a busy airport, dealing with arriving and departing flights, changing weather conditions, and strict safety regulations.

## Features

 * Real-time Simulation: The game runs on a "tick" system where time progresses, and aircraft states update automatically.
 * Dynamic Weather: Weather shifts between Clear, Rainy, and High Winds, forcing you to close specific runways dynamically.
 * Flight Planning: Procedurally generated flight plans with real airline codes (SWA, DAL, UAL), aircraft types, and routes.
 * Strict Rules: Penalties for clearing planes on closed runways or pushing back during boarding.
 * Win/Loss Condition: Accumulate points for successful operations. If you get 3 penalties, you're fired!
 * Cinematic Animations: ASCII art animations for fake loading screens as well as title and exit cards.

## Installation & Running
Ensure you have Rust and Cargo installed.
 * Clone the repository:
   git clone [https://github.com/mtepenner/The-Rust-y-ATC.git](https://github.com/mtepenner/The-Rust-y-ATC.git)
cd atc_simulator

 * Run the game:
   cargo run

## How to Play
Your goal is to manage aircraft states without causing accidents or violating airport protocols.
### Commands
| Command | Usage | Description |
|---|---|---|
| Pushback | pushback [id] | Clears a plane at the gate to taxi (e.g., pushback SWA123). |
| Takeoff | takeoff [id] [runway] | Clears a plane at the runway to depart (e.g., takeoff SWA123 24L). |
| Land | land [id] [runway] | Clears an incoming plane to land (e.g., land DAL456 18). |
| Wait | wait | Holds position for one tick (advances time). |
| Quit | quit | Exits the current session. |
### Scoring & Rules
 * +10 Points: Successful Takeoff or Landing.
 * +5 Points: Successful Pushback.
 * -5 Points: Penalty (Strike).
Game Over: If you accumulate 3 Strikes, the game ends immediately.
Common Penalties:
 * Trying to pushback a plane that is still "Boarding".
 * Clearing a plane for takeoff/landing on a runway closed by weather.
 * Interacting with a plane that doesn't exist.
## Project Structure
The project is modularized into several Rust files for maintainability and logic separation:
 * src/main.rs
   * Role: The Entry Point.
   * Function: Handles the main menu, the "How to Play" screen, the game loop, and user input handling. It ties all other modules together.
 * src/game.rs
   * Role: The Game Engine.
   * Function: Manages the global state (Score, Penalties, Game Over status). It processes user commands and updates the simulation "ticks."
 * src/airport.rs
   * Role: Environment Manager.
   * Function: Defines the Airport and Runway structs. Handles weather generation and logic (e.g., closing Runway 18 during Rain).
 * src/plane.rs
   * Role: The Actor.
   * Function: Defines the Plane struct and the State Machine (Boarding -> ReadyForPushback -> Taxiing, etc.). It handles individual aircraft timers.
 * src/flight_plan.rs
   * Role: Data Generation.
   * Function: Procedurally generates realistic flight data, including Origin/Destination airports, Waypoints, Airlines, and Aircraft Types.

## Dependencies
 * rand: Used for random number generation (Weather changes, flight ID generation, plane spawning).

### Everything seems to be operational.  I have experienced no major issues in final testing.  

## Lessons learned
* I learned to treat the Rust compiler as more of a strict mentor than the enemy.  I encountered quite a few type mismatches and missing field errors which helped to prevent bugs at runtime
* Making my code modular was also valuable for readability
* Developing the PlaneStatus enum allowed me to prevent planes from jumping to states that make no sense
* It was also reinstated to me the value in version control and helped me patch up my ability to do Git pulls whenever necessary
