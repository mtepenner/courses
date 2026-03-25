use rand::seq::SliceRandom; // <-- Removed 'Rng'

// --- AIRPORT MODULE ---
// Contains logic for the airport environment (weather, runways)

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weather {
    Clear,
    Rainy,
    HighWinds,
}

#[derive(Debug)]
pub struct Runway {
    pub id: String,
    pub is_open: bool,
}

pub struct Airport {
    pub weather: Weather,
    pub runways: Vec<Runway>,
}

impl Airport {
    pub fn new() -> Self {
        Airport {
            weather: Weather::Clear,
            runways: vec![
                Runway { id: "24L".to_string(), is_open: true },
                Runway { id: "24R".to_string(), is_open: true },
                Runway { id: "18".to_string(), is_open: true },
            ],
        }
    }

    /// Update weather and runway status
    pub fn update_weather(&mut self) {
        let patterns = [Weather::Clear, Weather::Rainy, Weather::HighWinds];
        self.weather = *patterns.choose(&mut rand::thread_rng()).unwrap();

        println!("WEATHER UPDATE: Weather is now {:?}", self.weather);

        // Apply weather-based rules
        for runway in self.runways.iter_mut() {
            match self.weather {
                Weather::Clear => runway.is_open = true,
                Weather::Rainy => {
                    // In rain, let's close one runway
                    if runway.id == "18" {
                        runway.is_open = false;
                        println!("RUNWAY INFO: Runway 18 closed due to rain.");
                    } else {
                        runway.is_open = true;
                    }
                }
                Weather::HighWinds => {
                    // In high winds, close crosswind runways
                    if runway.id.contains("24") {
                        runway.is_open = false;
                        println!("RUNWAY INFO: Runways 24L/R closed due to high winds.");
                    } else {
                        runway.is_open = true;
                    }
                }
            }
        }
    }

    pub fn get_runway(&self, id: &str) -> Option<&Runway> {
        // --- FIX is here ---
        self.runways.iter().find(|r| r.id.eq_ignore_ascii_case(id))
    }
}
