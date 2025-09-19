pub mod sample;
pub mod types;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use crate::{
        sample::OrderBookImpl,
        types::{Command, Side, TestCase},
    };

    #[test]
    fn basic_matching_test() {
        let test_case = TestCase::new(
            vec![
                Command::order(0, Side::Bid, 10, 10),
                Command::order(1, Side::Ask, 11, 11),
                Command::order(2, Side::Ask, 9, 9),
                Command::order(3, Side::Bid, 20, 20),
            ],
            9 + 11,
            10 * 9 + 11 * 11,
        );
        test_case.execute::<OrderBookImpl>();
    }

    #[test]
    fn basic_cancel_test() {
        let test_case = TestCase::new(
            vec![
                Command::order(0, Side::Bid, 10, 10),
                Command::order(1, Side::Ask, 11, 11),
                Command::order(2, Side::Ask, 9, 9),
                Command::order(3, Side::Bid, 20, 20),
                Command::cancel(0),
                Command::order(4, Side::Ask, 20, 1),
                Command::cancel(3),
            ],
            9 + 11 + 9,
            10 * 9 + 11 * 11 + 9 * 20,
        );
        test_case.execute::<OrderBookImpl>();
    }

    #[test]
    fn large_test() {
        let seed: u64 = 42;

        // Create a seeded RNG
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Generate some reproducible values
        const TOTAL: u64 = 1000000;
        let mut commands = vec![];
        for i in 0..TOTAL {
            let cancel_id = rng.random_range(0..2 * (i + 1));
            let is_buy = rng.random_bool(0.5);
            let side = if is_buy { Side::Bid } else { Side::Ask };
            let mut px = rng.random_range(20..30);
            let sz = rng.random_range(10..20);
            if is_buy {
                px -= 9;
            }
            let cmd = Command::order(i, side, sz, px);
            commands.push(cmd);
            commands.push(Command::cancel(cancel_id));
        }
        for i in 0..TOTAL {
            commands.push(Command::cancel(i));
        }
        let test_case = TestCase::new(commands, 720959, 14419180);
        test_case.execute::<OrderBookImpl>();
    }
}
