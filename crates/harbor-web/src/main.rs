fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let cargo = args.first().is_some_and(|arg| arg == "cargo");
    let result = args
        .get(usize::from(cargo))
        .cloned()
        .ok_or_else(|| "usage: harbor-browser [cargo] <query.json>".to_string())
        .and_then(|path| std::fs::read_to_string(path).map_err(|error| error.to_string()))
        .and_then(|input| {
            if cargo {
                harbor_web::cargo::evaluate(&input)
            } else {
                harbor_web::evaluate(&input)
            }
        });
    match result {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
