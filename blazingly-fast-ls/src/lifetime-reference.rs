fn main() {
    let word_a = String::from("greeting");
    {
        let result;
        let word_b = String::from("welcome");

        result = longest_word(&word_a, &word_b);
        println!("{:?}", result);
    }
}

fn longest_word<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
