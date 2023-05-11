pub fn get_input() -> (String, String, String) {
    let mut input: String = Default::default();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("Not enough inputs");
    }

    let from = args.get(1).unwrap();
    let to = args.get(2).unwrap();

    args.iter().skip(3).for_each(|x| {
        input.push(' ');
        input.push_str(x);
    });

    if input.is_empty() {
        panic!("Cant translate without any input.");
    }

    (from.to_string(), to.to_string(), input.trim().to_string())
}
