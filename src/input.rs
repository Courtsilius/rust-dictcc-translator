use crate::input_resolver::InputResolver;

pub fn get_input(v_input: &Vec<String>) -> (String, String, String) {
    let from = v_input.get(1).unwrap();
    let to = v_input.get(2).unwrap();

    let mut input: String = Default::default();
    v_input.iter().skip(3).for_each(|x| {
        input.push(' ');
        input.push_str(x);
    });

    if input.is_empty() {
        panic!("Cant translate without any input.");
    }

    (from.to_string(), to.to_string(), input.trim().to_string())
}

pub fn handle_input() -> InputResolver {
    let args: Vec<String> = std::env::args().collect();

    match &args.first().unwrap()[..2] {
        "--" => help(args.first()),
        _ => resolve(args),
    }
}

fn resolve(args: Vec<String>) -> InputResolver {
    if args.len() < 4 {
        help(args.first())
    } else {
        InputResolver::new(args, 1)
    }
}

fn help(cmd: Option<&String>) -> InputResolver {
    let help: Vec<String> = vec!["Hilfetext".to_string()];
    match cmd.unwrap().to_string() {
        //"--help" => InputResolver::new(help, 0),
        _ => InputResolver::new(help, 0),
    }
}
