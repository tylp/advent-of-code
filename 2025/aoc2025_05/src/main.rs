#[derive(Debug)]
struct Recipe {
    fresh_ingredients_range: Vec<Vec<u64>>,
    available_ingredients: Vec<u64>,
}

impl Recipe {
    pub fn new(lines: Vec<String>) -> Self {
        let (index, _blank) = lines
            .iter()
            .enumerate()
            .find(|line| line.1.is_empty())
            .expect("The recipe is malformed");

        let (left, right) = lines.split_at(index);

        let fresh_ingredients_range = left
            .iter()
            .map(|line| {
                let parts: Vec<_> = line.split("-").collect();

                let start = parts
                    .first()
                    .expect("Failed to get start of range")
                    .parse::<u64>()
                    .expect("Failed to parse start of range");
                let end = parts
                    .last()
                    .expect("Failed to get end of range")
                    .parse::<u64>()
                    .expect("Failed to parse end of range");

                (start..=end).collect()
            })
            .collect();

        let available_ingredients = right
            .iter()
            .map(|line| {
                line.parse::<u64>()
                    .expect("Failed to parse available ingredient")
            })
            .collect();

        Self {
            fresh_ingredients_range,
            available_ingredients,
        }
    }
}

fn main() {
    let recipe = Recipe::new(aoc::init());

    println!("Built a new recipe: {recipe:?}");
}

#[cfg(test)]
mod tests {}
