use rand::Rng;

pub fn generate_short_url() -> String {
    let mut rng = rand::thread_rng();
    let short_url: String = (0..6)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)  // a-z random words
        .collect();
    short_url
}