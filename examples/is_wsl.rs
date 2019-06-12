fn main() {
    if wsl::is_wsl() {
        println!("Yes");
    } else {
        println!("No");
    }
}
