// fn main() {
//     greet("World");
//     let sum = add(2, 3);
//     println!("Suma: {}", sum);
//     test();
//     let num = 4;
//     if is_even(num) {
//         println!("Numărul {} este par!", num);
//     } else {
//         println!("Numărul {} este impar!", num);
//     }
// }

// fn greet(name: &str) {  // Parametru de tip string slice
//     println!("Hello, {}!", name);
// }

// fn add(a: i32, b: i32) -> i32 {  // Returnează un i32
//     a + b  // Fără `;` = expresie de return
// }

// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         true  // Return implicit
//     } else {
//         false
//     }
// }

// fn increment(num: &mut i32) {
//     *num += 1;  // Dereferențiere cu `*`
// }

// fn test() {
//     let mut x = 5;
//     increment(&mut x);
//     println!("x = {}", x); // x = 6
// }

// Definim nivelurile de energie ca o enumerare (enum)
// Asta ne permite să avem valori predefinite și clare.
// Definim nivelurile de energie ca o enumerare (enum)
// Asta ne permite să avem valori predefinite și clare.
enum EnergyLevel {
    LOW,
    MEDIUM,
    HIGH,
}

// Definim structura Passion pentru a indica dacă pasiunea a fost redescoperită
// Conține un câmp boolean care ne va ajuta să știm starea pasiunii
struct Passion {
    rediscovered: bool,
}

impl Passion {
    // O metodă ajutătoare pentru a crea o instanță de Passion
    // unde "rediscovered" este setat la true.
    fn rediscovered() -> Self {
        Passion { rediscovered: true }
    }

    // O metodă ajutătoare pentru a crea o instanță de Passion
    // unde "rediscovered" este setat la false.
    fn not_yet_rediscovered() -> Self {
        Passion { rediscovered: false }
    }

    // O metodă pentru a verifica starea pasiunii
    fn is_rediscovered(&self) -> bool {
        self.rediscovered
    }
}

// Funcția principală a programului
fn main() {
    // Aici ar trebui să ai o modalitate de a obține nivelul curent de energie.
    // Pentru exemplul ăsta, îl vom seta manual, dar într-un program real
    // ar putea veni dintr-o intrare de la utilizator, un senzor, etc.
    let mut current_energy_level = EnergyLevel::LOW; // Începem cu LOW

    loop {
        let passion = match current_energy_level {
            EnergyLevel::LOW => take_break(),
            EnergyLevel::MEDIUM => code_for_fun(),
            EnergyLevel::HIGH => dive_deep(),
        };

        if passion.is_rediscovered() {
            println!("Pasiunea a fost redescoperită! Ieșim din buclă.");
            break; // Ieșim din bucla `loop` dacă pasiunea e redescoperită
        } else {
            // Dacă pasiunea nu a fost redescoperită, putem schimba nivelul de energie
            // pentru următoarea iterație. Aici poți implementa o logică mai complexă.
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
    Passion::rediscovered() // Presupunem că te ajută să-ți redescoperi pasiunea
}

fn dive_deep() -> Passion {
    println!("Te scufunzi adânc în cod. Ești în elementul tău!");
    // Când ești la nivel maxim de energie, e cel mai probabil să-ți redescoperi pasiunea.
    Passion::rediscovered()
}


fn simulate_energy_change(current_level: &EnergyLevel) -> EnergyLevel {
    match current_level {
        EnergyLevel::LOW => EnergyLevel::MEDIUM, // După o pauză, energia ar putea crește
        EnergyLevel::MEDIUM => EnergyLevel::HIGH, // După ceva cod de fun, energia crește și mai mult
        EnergyLevel::HIGH => EnergyLevel::LOW, // Un ciclu complet ar putea duce la epuizare
    }
}
