use rand::Rng;
use std::collections::HashMap;

fn parse(text: &str) -> HashMap<(&str, &str), Vec<&str>> {
    let mut words = text.split(" ");

    let mut table: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    let mut w1 = words.next().unwrap();

    let mut w2 = words.next().unwrap();

    for w3 in words {
        if let Some(vec) = table.get_mut(&(w1, w2)) {
            vec.push(w3);
        } else {
            table.insert((w1, w2), vec![w3]);
        }

        w1 = w2;
        w2 = w3;
    }

    table
}

fn text_from_table(table: HashMap<(&str, &str), Vec<&str>>, length: u32) -> String {
    let mut rng = rand::thread_rng();
    let keys = table.keys().collect::<Vec<&(&str, &str)>>();

    let (mut w1, mut w2) = keys[rng.gen_range(0..keys.len())];

    let mut output = format!("{} {}", w1, w2); 

    for _ in 0..length-2 {
        match table.get(&(w1, w2)) {
            Some(values) => {
                let w3 = values[rng.gen_range(0..values.len())];

                output = format!("{} {}", output, w3);

                w1 = w2;
                w2 = w3;
            },
            None => break,
        }
    }

    output
}

pub fn gen_text(text: &str, length: u32) -> String {
    text_from_table(parse(text), length)
}