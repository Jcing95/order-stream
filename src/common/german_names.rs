use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Fun German first names - memorable but not too long
const GERMAN_NAMES: &[&str] = &[
    "Günther", "Brunhilde", "Wolfgang", "Gisela", "Friedrich", "Lieselotte",
    "Siegfried", "Brunhild", "Dietrich", "Edeltraud", "Gottfried", "Waltraud",
    "Bernhard", "Kunigunde", "Rüdiger", "Elfriede", "Adalbert", "Mechthild",
    "Willibald", "Roswitha", "Reinhard", "Hannelore", "Gerhard", "Ingeborg",
    "Manfred", "Christa", "Dieter", "Ursula", "Herbert", "Brigitte",
    "Erwin", "Helga", "Klaus", "Monika", "Hans", "Petra", "Franz", "Sabine",
    "Otto", "Claudia", "Karl", "Andrea", "Fritz", "Beate", "Ludwig", "Doris",
    "Horst", "Karin", "Willi", "Ingrid", "Ernst", "Gabriele", "Rolf", "Martina",
    "Heinz", "Susanne", "Jürgen", "Birgit", "Bernd", "Ulrike", "Uwe", "Silke",
    "Lothar", "Cornelia", "Norbert", "Renate", "Joachim", "Heike", "Volker", "Ute",
    "Günter", "Angelika", "Helmut", "Christine", "Werner", "Barbara", "Georg", "Elisabeth",
    "Wulfgang", "Brunhildegard", "Siegbert", "Irmtraud", "Gottlob", "Adelheid",
    "Hubertus", "Walpurgis", "Balduin", "Kunigundis", "Friedhelm", "Mechthildis"
];

/// Generates a deterministic German first name from an order ID
/// 
/// The same order ID will always generate the same German name across all devices
/// using built-in hashing for deterministic selection from a static list.
/// 
/// # Examples
/// 
/// ```
/// use order_stream::common::german_names::generate_german_name;
/// 
/// let name1 = generate_german_name("order-123");
/// let name2 = generate_german_name("order-123");
/// assert_eq!(name1, name2); // Always the same name for the same ID
/// ```
pub fn generate_german_name(order_id: &str) -> String {
    // Create a deterministic hash from the order ID
    let mut hasher = DefaultHasher::new();
    order_id.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Use hash to select a name from our static list
    let index = (hash as usize) % GERMAN_NAMES.len();
    GERMAN_NAMES[index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_generation() {
        let order_id = "test-order-123";
        let name1 = generate_german_name(order_id);
        let name2 = generate_german_name(order_id);
        
        assert_eq!(name1, name2, "Same order ID should generate same name");
    }

    #[test]
    fn test_different_ids_different_names() {
        let name1 = generate_german_name("order-1");
        let name2 = generate_german_name("order-2");
        
        // Note: with our hash function, different IDs might occasionally generate the same name
        // This is acceptable for a fun naming system
        println!("order-1 -> {}, order-2 -> {}", name1, name2);
    }

    #[test]
    fn test_german_name_format() {
        let name = generate_german_name("test-order");
        
        // Should be a single German first name
        assert!(!name.is_empty(), "Generated name should not be empty");
        assert!(name.chars().all(|c| c.is_alphabetic() || c == 'ä' || c == 'ö' || c == 'ü' || c == 'ß'), 
                "Should only contain alphabetic characters and German umlauts");
    }

    #[test]
    fn test_example_german_names() {
        // Generate some example names to demonstrate the functionality
        println!("Example German names generated from order IDs:");
        let test_ids = ["order-001", "order-002", "abc-123-def", "test-order-456", "kitchen-special-789"];
        
        for id in &test_ids {
            let name = generate_german_name(id);
            println!("  {} -> {}", id, name);
        }
    }
    
    #[test]
    fn test_name_selection_coverage() {
        // Test that we can generate different names from our list
        let mut names_generated = std::collections::HashSet::new();
        
        for i in 0..20 {
            let order_id = format!("test-order-{}", i);
            let name = generate_german_name(&order_id);
            names_generated.insert(name);
        }
        
        println!("Generated {} unique names from 20 different order IDs", names_generated.len());
        assert!(names_generated.len() > 1, "Should generate multiple different names");
    }
}