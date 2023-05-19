fn main() {
    // predefined action words
    let action_keywords = vec![
        "psychic powers",
        "motorcycle chase",
        "government conspiracy",
        "post-apocalyptic",
        "neo-Tokyo",
        "telekinesis",
        "mutation"
    ];

    // input action keyword
    let input_keyword = "motorcycle chase";

    if action_keywords.contains(&input_keyword) {
        println!("Input keyword matches an action: {}", input_keyword);
    } else {
        println!("Input keyword don't matches any action: ");
    }
}