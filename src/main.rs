use jason::parse_from_str;

fn main() {
    println!("{}", parse_from_str("01").unwrap_err());
    println!("{}", parse_from_str("{true: 1}").unwrap_err());
    println!("{}", parse_from_str("[1, 2,]").unwrap_err());
}
