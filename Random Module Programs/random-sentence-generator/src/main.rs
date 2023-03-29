use rand::Rng;

fn main() {
    let subjects = ["cat", "dog", "bird", "fish", "hamster", "capybara"];
    let verbs = ["eats", "sleeps", "jumps", "runs", "hops"];
    let objects = ["mouse", "food", "ball", "tree", "carrot"];

    let mut rng = rand::thread_rng();
    let sentence = format!("{} {} with a {}.",
    subjects[rng.gen_range(0..subjects.len())],
    verbs[rng.gen_range(0..verbs.len())],
    objects[rng.gen_range(0..objects.len())]
);
println!("The {}", sentence);
}