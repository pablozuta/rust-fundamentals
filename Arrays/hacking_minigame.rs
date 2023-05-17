struct Hack {
    level: u8,
    progress: u32,
    success_chance: f32,
}

impl Hack {
    fn new(level: u8,  success_chance: f32) -> Self {
        Hack {
            level,
            progress: 0,
            success_chance,
        }
    }
    fn increase_progress(&mut self, amount: u32) {
        self.progress += amount;
    }
    fn display_info(&self) {
        println!("Hack Level: {}", self.level);
        println!("Progress: {}/100", self.progress);
        println!("Success Chance: {:.2}%", self.success_chance);
        println!();
    }
}

fn main() {
    // simulate hacking minigame
    let mut hack = Hack::new(5, 75.0);

    hack.display_info();

    // simulate progress increase
    hack.increase_progress(20);

    hack.display_info();
}