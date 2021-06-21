use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School(HashMap<u32, Vec<String>>);

impl School {
    #[allow(clippy::new_without_default)]
    pub fn new() -> School {
        School(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade = self.0.entry(grade).or_default();
        grade.push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g: Vec<u32> = self.0.keys().copied().collect();
        g.sort_unstable();
        g
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v = self.0.get(&grade).cloned().unwrap_or_default();
        v.sort_unstable();
        v
    }
}
