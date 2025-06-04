enum EnergyLevel {
    LOW,
    MEDIUM,
    HIGH,
}

struct Passion {
    rediscovered: bool,
}

impl Passion {
    fn rediscovered() -> Self {
        Passion { rediscovered: true }
    }

    fn not_yet_rediscovered() -> Self {
        Passion { rediscovered: false }
    }

    fn is_rediscovered(&self) -> bool {
        self.rediscovered
    }
}

fn main() {
    let mut current_energy_level = EnergyLevel::LOW;

    loop {
        let passion = match current_energy_level {
            EnergyLevel::LOW => take_break(),
            EnergyLevel::MEDIUM => code_for_fun(),
            EnergyLevel::HIGH => dive_deep(),
        };

        if passion.is_rediscovered() {
            println!("Pasiunea a fost redescoperită! Ieșim din buclă.");
            break;
        } else {
            println!("Pasiunea încă nu e complet redescoperită. Continuăm...");
            current_energy_level = simulate_energy_change(&current_energy_level);
        }
    }
}

fn take_break() -> Passion {
    println!("Ai nevoie de o pauză, bai boss!");
    Passion::not_yet_rediscovered()
}

fn code_for_fun() -> Passion {
    println!("Codezi de plăcere, ce tare!");
    Passion::rediscovered()
}

fn dive_deep() -> Passion {
    println!("Te scufunzi adânc în cod. Ești în elementul tău!");
    Passion::rediscovered()
}

fn simulate_energy_change(current_level: &EnergyLevel) -> EnergyLevel {
    match current_level {
        EnergyLevel::LOW => EnergyLevel::MEDIUM,
        EnergyLevel::MEDIUM => EnergyLevel::HIGH,
        EnergyLevel::HIGH => EnergyLevel::LOW,
    }
}