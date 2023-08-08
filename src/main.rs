fn main() {
    let mut command = String::new();

    loop{
        std::io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");
    }
}
