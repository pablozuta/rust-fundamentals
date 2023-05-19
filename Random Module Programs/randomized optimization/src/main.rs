use rand::Rng;

// Fitness function to be optimized
fn fitness(x: f64) -> f64 {
    x.powi(2) - 3.0 * x + 4.0
}

// Generate a random neighbor within a given range of the current value
fn random_neighbor(x: f64, range: f64) -> f64 {
    let mut rng = rand::thread_rng();
    x + rng.gen_range(-range..range)
}

// Randomized Hill Climbing algorithm
fn randomized_hill_climbing(mut x: f64, range: f64, steps: usize) -> f64 {
    for _ in 0..steps {
        // Generate a random neighbor
        let neighbor = random_neighbor(x, range);
        
        // If the neighbor has a better fitness value, move to that neighbor
        if fitness(neighbor) < fitness(x) {
            x = neighbor;
        }
    }
    x
}

fn main() {
    let x0 = 0.0;       // Initial value of x
    let range = 0.1;    // Range within which random neighbors are generated
    let steps = 1000;   // Number of steps in the optimization process
    
    // Run the Randomized Hill Climbing algorithm and obtain the optimal x
    let x_opt = randomized_hill_climbing(x0, range, steps);

    println!("Optimal x: {}", x_opt);
    println!("Optimal fitness: {}", fitness(x_opt));
}
