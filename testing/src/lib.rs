#[derive(Debug, PartialEq, Eq)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

// cfg: configuration
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        // let mut museum = crate::Museum::new();
        // let mut museum = super::Museum::new();

        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
        assert_eq!(
            museum.revenue, 25,
            "The revenue from selling 1 ticket did not match expectations."
        );
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        assert!(!museum.has_impressive_collection());
        museum.buy_painting("Girl with a Pearl Earring");
        assert!(museum.has_impressive_collection(), "The museum did not have an impressive painting, despite having more than two paintings.");
    }

    // #[test]
    // fn hashmaps() {
    //     let mut one = HashMap::new();
    //     one.insert("A", 2);
    //     one.insert("B", 3);

    //     let mut two = HashMap::new();
    //     two.insert("B", 3);
    //     two.insert("C", 4);

    //     assert_eq!(one, two);
    // }

    #[test]
    fn new_museums_are_equal() {
        let mut museum1 = Museum::new();
        museum1.sell_ticket();

        let mut museum2 = Museum::new();
        museum2.sell_ticket();

        assert_eq!(
            museum1, museum2,
            "Two new museum instances were not found to be equal: {museum1:?} // {museum2:?}"
        );
    }
}
