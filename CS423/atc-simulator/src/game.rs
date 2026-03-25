// Use `crate::` to import from other files in our project
use crate::airport::Airport;
use crate::flight_plan::FlightPlanManager;
use crate::plane::{Plane, PlaneStatus};
use rand::Rng;

// --- GAME MODULE ---
// Contains the main game state and logic

pub struct Game {
    pub score: i32,
    pub penalty_strikes: u32,
    pub airport: Airport,
    pub planes: Vec<Plane>,
    pub flight_plan_manager: FlightPlanManager, // Field exists
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut flight_plan_manager = FlightPlanManager::new();
        // Generate a couple of initial flight plans
        let plan1 = flight_plan_manager.generate_new_plan();
        let plan2 = flight_plan_manager.generate_new_plan();

        // --- THIS IS THE CORRECTED STRUCT INITIALIZATION ---
        Game {
            score: 0,
            penalty_strikes: 0,
            airport: Airport::new(),
            planes: vec![
                Plane::new(plan1), // Use plans to create planes
                Plane::new(plan2),
            ],
            flight_plan_manager, // <-- THE FIX: Added this missing field
            game_over: false,
        }
    }

    /// The main game "tick". Update all entities.
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Update all planes
        for plane in self.planes.iter_mut() {
            plane.update();
        }

        // Remove planes that are at the gate (finished)
        self.planes.retain(|p| {
            if p.status == PlaneStatus::AtGate {
                // If plane is done, remove its flight plan from the manager
                self.flight_plan_manager.remove_plan(&p.id);
                false
            } else {
                true
            }
        });

        // Occasionally spawn new planes
        if rand::thread_rng().gen_bool(0.1) { // 10% chance each tick
            self.spawn_plane();
        }

        // Occasionally change weather
        if rand::thread_rng().gen_bool(0.05) { // 5% chance each tick
            self.airport.update_weather();
        }
    }

    fn spawn_plane(&mut self) {
        // Use the manager to create a new plan
        let new_plan = self.flight_plan_manager.generate_new_plan();
        let mut new_plane = Plane::new(new_plan); // Correctly passes FlightPlan
        
        // 50/50 chance to spawn arriving or departing
        if rand::thread_rng().gen_bool(0.5) {
            new_plane.status = PlaneStatus::InAir;
            new_plane.timer = rand::thread_rng().gen_range(10..=20); // Time until ready to land
            println!(
                "NEW PLANE: {} is approaching ({} -> {}), will be ready to land soon.",
                new_plane.id, new_plane.flight_plan.origin, new_plane.flight_plan.destination
            );
        } else {
            println!(
                "NEW PLANE: {} is at the gate ({} -> {}), beginning boarding.",
                new_plane.id, new_plane.flight_plan.origin, new_plane.flight_plan.destination
            );
        }
        self.planes.push(new_plane);
    }

    /// Print the current game state to the console
    pub fn draw(&self) {
        if self.game_over {
            return;
        }
        
        println!("\n-----------------------------------------------------");
        println!("SCORE: {} | PENALTIES: {}/3 | WEATHER: {:?}", self.score, self.penalty_strikes, self.airport.weather);
        
        println!("\n--- RUNWAYS ---");
        for runway in &self.airport.runways {
            let status = if runway.is_open { "OPEN" } else { "CLOSED" };
            println!("  - Runway {}: {}", runway.id, status);
        }

        println!("\n--- AIRCRAFT ---");
        for plane in &self.planes {
            let timer_info = if plane.timer > 0 { format!(" ({} ticks left)", plane.timer) } else { "".to_string() };
            println!(
                "  - {}: {:?}{} | {}",
                plane.id,
                plane.status,
                timer_info,
                plane.flight_plan.summary()
            );
        }
        
        println!("\n--- AWAITING YOUR COMMAND ---");
        println!("Commands: pushback [id] | taxi [id] | takeoff [id] [runway] | land [id] [runway] | wait | quit");
    }

    /// Handle user input
    pub fn process_command(&mut self, command: String) {
        if self.game_over {
            return;
        }
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let command = parts[0];
        let plane_id = parts.get(1).map(|s| s.to_uppercase());
        let runway_id = parts.get(2).map(|s| s.to_uppercase());

        // Find the plane
        let plane_opt = self.planes.iter_mut().find(|p| p.id.eq_ignore_ascii_case(plane_id.as_deref().unwrap_or_default()));


        if plane_opt.is_none() && command != "quit" && command != "wait" {
             // Adding "wait" command to allow game to tick without action
            println!("COMMAND ERROR: Plane not found: {:?}", plane_id);
            if !parts.is_empty() && command != "" {
                 self.add_penalty();
            }
            return;
        }


        match command {
            "pushback" => {
                let plane = plane_opt.unwrap(); // Safe now
                if plane.status == PlaneStatus::ReadyForPushback {
                    println!("ATC: {} cleared for pushback.", plane.id);
                    plane.status = PlaneStatus::TaxiingToRunway;
                    plane.timer = 15; // 15 ticks to taxi
                    self.add_score(5); // Half points
                } else if plane.status == PlaneStatus::Boarding {
                    println!("PENALTY: {} is still boarding! Can't pushback!", plane.id);
                    self.add_penalty();
                } else {
                    println!("PENALTY: {} is not ready for pushback.", plane.id);
                    self.add_penalty();
                }
            }
            "takeoff" => {
                let plane = plane_opt.unwrap(); // Safe now
                let rwy_id = match runway_id {
                    Some(id) => id,
                    None => {
                        println!("PENALTY: Must specify runway for takeoff.");
                        self.add_penalty();
                        return;
                    }
                };
                
                if plane.status != PlaneStatus::ReadyForTakeoff {
                    println!("PENALTY: {} is not ready for takeoff.", plane.id);
                    self.add_penalty();
                    return;
                }

                match self.airport.get_runway(&rwy_id) {
                    Some(runway) if runway.is_open => {
                        println!("ATC: {} cleared for takeoff, runway {}.", plane.id, runway.id);
                        plane.status = PlaneStatus::InAir;
                        plane.timer = 30; // 30 ticks until next event (e.g., ready to land)
                        self.add_score(10); // Full points
                    }
                    Some(runway) => { // Runway exists but is not open
                         println!("PENALTY: Runway {} is CLOSED! You can't clear {} for takeoff!", runway.id, plane.id);
                         self.add_penalty();
                    }
                    None => { // Runway doesn't exist
                        println!("PENALTY: Runway {} does not exist!", rwy_id);
                        self.add_penalty();
                    }
                }
            }
            "land" => {
                let plane = plane_opt.unwrap(); // Safe now
                let rwy_id = match runway_id {
                    Some(id) => id,
                    None => {
                        println!("PENALTY: Must specify runway for landing.");
                        self.add_penalty();
                        return;
                    }
                };
                
                if plane.status != PlaneStatus::ReadyToLand {
                    println!("PENALTY: {} is not ready to land.", plane.id);
                    self.add_penalty();
                    return;
                }

                match self.airport.get_runway(&rwy_id) {
                    Some(runway) if runway.is_open => {
                        println!("ATC: {} cleared to land, runway {}.", plane.id, runway.id);
                        plane.status = PlaneStatus::TaxiingToGate;
                        plane.timer = 10; // 10 ticks to taxi to gate
                        self.add_score(10); // Full points
                    }
                    Some(runway) => {
                         println!("PENALTY: Runway {} is CLOSED! You can't clear {} to land!", runway.id, plane.id);
                         self.add_penalty();
                    }
                    None => {
                        println!("PENALTY: Runway {} does not exist!", rwy_id);
                        self.add_penalty();
                    }
                }
            }
            "taxi" => {
                // In this simple version, "taxi" is handled by pushback.
                // You could expand this to require a "taxi" command.
                println!("INFO: 'pushback' command handles taxiing to runway.");
            }
            "wait" => {
                // Do nothing, just let the game tick
                println!("ATC: Holding position.");
            }
            "quit" => {
                self.game_over = true;
                println!("Thanks for playing!");
            }
            _ => {
                if !command.is_empty() {
                    println!("Unknown command: {}", command);
                }
            }
        }
    }

    fn add_score(&mut self, amount: i32) {
        self.score += amount;
        // Reset penalty strikes on a successful command
        self.penalty_strikes = 0;
        println!("Score: {}", self.score);
    }

    fn add_penalty(&mut self) {
        self.score -= 5;
        self.penalty_strikes += 1;
        println!("PENALTY! Score: {}, Strikes: {}/3", self.score, self.penalty_strikes);

        if self.penalty_strikes >= 3 {
            println!("\n******************************************");
            println!("Alright, get out, you're fired.");
            println!("FINAL SCORE: {}", self.score);
            println!("******************************************");
            self.game_over = true;
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
