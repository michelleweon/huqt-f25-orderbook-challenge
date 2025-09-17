pub mod sample;
pub mod types;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{
        sample::OrderBookImpl,
        types::{Command, Side, TestCase},
    };

    #[test]
    fn basic_matching_test() {
        let test_case = TestCase::new(
            vec![
                Command::AddOrder {
                    id: 0,
                    side: Side::Bid,
                    size: 10,
                    price: 10,
                },
                Command::AddOrder {
                    id: 1,
                    side: Side::Ask,
                    size: 11,
                    price: 11,
                },
                Command::AddOrder {
                    id: 2,
                    side: Side::Ask,
                    size: 9,
                    price: 9,
                },
                Command::AddOrder {
                    id: 3,
                    side: Side::Bid,
                    size: 20,
                    price: 20,
                },
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
                Command::AddOrder {
                    id: 0,
                    side: Side::Bid,
                    size: 10,
                    price: 10,
                },
                Command::AddOrder {
                    id: 1,
                    side: Side::Ask,
                    size: 11,
                    price: 11,
                },
                Command::AddOrder {
                    id: 2,
                    side: Side::Ask,
                    size: 9,
                    price: 9,
                },
                Command::AddOrder {
                    id: 3,
                    side: Side::Bid,
                    size: 20,
                    price: 20,
                },
                Command::CancelOrder { id: 0 },
                Command::AddOrder {
                    id: 4,
                    side: Side::Ask,
                    size: 20,
                    price: 1,
                },
            ],
            9 + 11 + 9,
            10 * 9 + 11 * 11 + 9 * 20,
        );
        test_case.execute::<OrderBookImpl>();
    }
}
