use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;

pub fn rand_name() -> String {
    // Setup a random name so CI testing won't cause collisions
    let mut rng = thread_rng();
    let mut name = String::from("ruarango-");
    let name_ext: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    name.push_str(&name_ext);
    name
}
