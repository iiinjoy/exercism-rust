use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words: HashMap<&str, u32> = HashMap::new();

    magazine.iter().for_each(|w| {
        magazine_words.entry(w).and_modify(|n| *n += 1).or_insert(1);
    });

    note.iter()
        .try_for_each(|w| {
            use std::collections::hash_map::Entry;
            if let Entry::Occupied(mut o) = magazine_words.entry(w) {
                *o.get_mut() -= 1;
                if *o.get() == 0 {
                    o.remove();
                }
                Ok(())
            } else {
                Err(())
            }
        })
        .is_ok()
}
