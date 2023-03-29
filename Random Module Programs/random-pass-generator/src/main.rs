use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let mut rng = rand::thread_rng();
    let password: String = (0..20).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char }).collect();


    println!("Your password is: {}", password);
}