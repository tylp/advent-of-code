fn main() {
    resolve();
}

fn resolve() -> u16 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(), 5);
    }
}
