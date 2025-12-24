fn main() {
    let lines = aoc::init();

    // Find the adrress of the
    let mut separator = lines.split(|line| line.is_empty());
    let ranges = separator.next().expect("Failed to get ranges");
    let ingredients = separator.next().expect("Failed to get ingredients");

    let recipe = Recipe {
        ingredients,
        ranges,
    };

    let number_of_fresh_ingredient_ids = recipe.count_fresh_ingredients_ids();
    println!("Number of fresh ingredient ids is {number_of_fresh_ingredient_ids}");
}

struct Recipe<'a> {
    ranges: &'a [String],
    ingredients: &'a [String],
}

impl<'a> Recipe<'a> {
    fn count_fresh_ingredients_ids(&self) -> usize {
        let mut fresh_ingredients = 0u64;

        for ingredient in self.ingredients {
            for range in self.ranges {
                let ranges: Vec<&str> = range.split("-").collect();
                let range_start = ranges
                    .first()
                    .expect("Failed to get range start")
                    .parse::<u64>()
                    .expect("Failed to parse range start");
                let range_end = ranges
                    .last()
                    .expect("Failed to get range end")
                    .parse::<u64>()
                    .expect("Failed to parse range end");

                let ingredient = ingredient
                    .parse::<u64>()
                    .expect("Failed to parse ingredient id");

                if ingredient >= range_start && ingredient <= range_end {
                    fresh_ingredients += 1;
                    break;
                }
            }
        }

        fresh_ingredients as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::Recipe;

    /// Test the part 1 example with the following input :
    /// 3-5
    /// 10-14
    /// 16-20
    /// 12-18
    ///
    /// 1
    /// 5
    /// 8
    /// 11
    /// 17
    /// 32
    #[test]
    fn count_fresh_ingredients_ids_test() {
        let ranges = ["3-5", "10-14", "16-20", "12-18"].map(str::to_string);
        let ingredients = ["1", "5", "8", "11", "17", "32"].map(str::to_string);

        let recipe = Recipe {
            ranges: &ranges,
            ingredients: &ingredients,
        };

        let fresh = recipe.count_fresh_ingredients_ids();
        assert_eq!(fresh, 3);
    }
}
