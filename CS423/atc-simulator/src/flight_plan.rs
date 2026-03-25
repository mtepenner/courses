use rand::{seq::SliceRandom, Rng};

// --- AIRLINE ENUM ---
// Represents the airlines in the game
#[derive(Debug, Clone, Copy)]
pub enum Airline {
    Southwest,
    Delta,
    American,
    United,
    JetBlue,
}

impl Airline {
    /// Get the ICAO code for the airline
    fn get_id(&self) -> &str {
        match self {
            Airline::Southwest => "SWA",
            Airline::Delta => "DAL",
            Airline::American => "AAL",
            Airline::United => "UAL",
            Airline::JetBlue => "JBU",
        }
    }

    /// Select a random airline
    fn random() -> Self {
        let choices = [
            Airline::Southwest,
            Airline::Delta,
            Airline::American,
            Airline::United,
            Airline::JetBlue,
        ];
        *choices.choose(&mut rand::thread_rng()).unwrap()
    }
}

// --- FLIGHT TYPE ENUM ---
#[derive(Debug, Clone, Copy)]
pub enum FlightType {
    Domestic,
    International,
    Cargo,
}

impl FlightType {
    fn random() -> Self {
        let choices = [
            FlightType::Domestic,
            FlightType::Domestic, // Weight domestic heavier
            FlightType::International,
            FlightType::Cargo,
        ];
        *choices.choose(&mut rand::thread_rng()).unwrap()
    }
}

// --- WAYPOINT STRUCT ---
// A single point in a flight path
#[derive(Debug, Clone)]
#[allow(dead_code)] // <-- ADD THIS LINE to suppress warnings
pub struct Waypoint {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

// --- FLIGHT PLAN STRUCT ---
// Contains all data for a single flight
#[derive(Debug, Clone)]
#[allow(dead_code)] // <-- ADD THIS LINE to suppress warnings
pub struct FlightPlan {
    pub flight_id: String,
    pub airline: Airline,
    pub aircraft_type: String,
    pub origin: String,
    pub destination: String,
    pub flight_type: FlightType,
    pub route: Vec<Waypoint>,
}

impl FlightPlan {
    /// Create a new, randomized flight plan
    pub fn new() -> Self {
        let airline = Airline::random();
        let flight_num = rand::thread_rng().gen_range(100..=1999);
        let flight_id = format!("{}{}", airline.get_id(), flight_num);

        let aircraft_choices = ["B737", "A320", "B787", "A330", "E175"];
        let aircraft_type = aircraft_choices
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        let origin_choices = ["KPDX"];
        let dest_choices = ["KSFO", "KLAS", "KMIA", "KBOS", "KDEN"];

        let origin = origin_choices
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        let destination = dest_choices
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        FlightPlan {
            flight_id,
            airline,
            aircraft_type,
            origin,
            destination,
            flight_type: FlightType::random(),
            route: FlightPlan::generate_random_route(),
        }
    }

    /// Generate a simple, random route
    fn generate_random_route() -> Vec<Waypoint> {
        let mut route = Vec::new();
        let point_names = ["ALPHA", "BRAVO", "CHARLIE", "DELTA", "ECHO"];
        let num_points = rand::thread_rng().gen_range(2..=5);

        for _ in 0..num_points {
            route.push(Waypoint {
                name: point_names
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string(),
                latitude: rand::thread_rng().gen_range(30.0..=50.0),
                longitude: rand::thread_rng().gen_range(-120.0..=-75.0),
            });
        }
        route
    }

    /// Print a formatted flight plan summary
    pub fn summary(&self) -> String {
        format!(
            "{:<8} ({}) {} -> {} | {}",
            self.flight_id,
            self.aircraft_type,
            self.origin,
            self.destination,
            format!("{:?}", self.flight_type).to_uppercase()
        )
    }
}

// --- FLIGHT PLAN MANAGER ---
// A struct to hold and manage all active flight plans
pub struct FlightPlanManager {
    pub active_plans: Vec<FlightPlan>,
}

impl FlightPlanManager {
    pub fn new() -> Self {
        FlightPlanManager {
            active_plans: Vec::new(),
        }
    }

    /// Generate a new, unique flight plan
    pub fn generate_new_plan(&mut self) -> FlightPlan {
        // Ensure flight ID is unique (basic implementation)
        let mut new_plan = FlightPlan::new();
        while self
            .active_plans
            .iter()
            .any(|p| p.flight_id == new_plan.flight_id)
        {
            new_plan = FlightPlan::new();
        }
        let plan_clone = new_plan.clone();
        self.active_plans.push(new_plan);
        plan_clone
    }

    /// Remove a plan by ID
    pub fn remove_plan(&mut self, flight_id: &str) {
        self.active_plans.retain(|p| p.flight_id != flight_id);
    }
}
