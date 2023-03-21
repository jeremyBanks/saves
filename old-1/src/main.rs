fn main() {
    let saves = celeste::Saves::load_all()
        .into_iter()
        .next()
        .expect("to find Celeste save data");

    println!("{:?}", saves.saves[0]);
}
