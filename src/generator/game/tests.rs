#[cfg(test)]
mod tests_int {
    use crate::generator::game::{new, new_testing};

    #[test]
    fn seeding_test_generate_testing_game() {
        let original_result = new_testing(Some([1; 16]));

        for _i in 1..1000 {
            let game = new_testing(Some([1; 16]));
            assert_eq!(original_result, game);
        }
    }

    #[test]
    fn seeding_test_generate_new_game() {
        let original_result = new(Some([1; 16]));

        for _i in 1..1000 {
            let game = new(Some([1; 16]));
            assert_eq!(original_result, game);
        }
    }
}
