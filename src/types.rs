#[derive(Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    pub fn opp(&self) -> Self {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

pub trait OrderBook {
    // create new book
    fn new_book() -> Self;
    // insets order into the orderbook
    fn add_order(&mut self, id: u64, side: Side, size: i64, price: i64);
    // cancel resting order. Does nothing if order doesn't exist
    fn cancel_order(&mut self, id: u64);
    // total traded volume
    fn get_vol(&self) -> i64;
    // total traded notional volume
    fn get_ntnl_vol(&self) -> i64;
}

pub enum Command {
    CancelOrder {
        id: u64,
    },
    AddOrder {
        id: u64,
        side: Side,
        size: i64,
        price: i64,
    },
}

impl Command {
    pub fn order(id: u64, side: Side, size: i64, price: i64) -> Self {
        Self::AddOrder {
            id,
            side,
            size,
            price,
        }
    }
    pub fn cancel(id: u64) -> Self {
        Self::CancelOrder { id }
    }
}

pub struct TestCase {
    cmds: Vec<Command>,
    expected_vol: i64,
    expected_ntnl_vol: i64,
}

impl TestCase {
    pub fn new(cmds: Vec<Command>, expected_vol: i64, expected_ntnl_vol: i64) -> Self {
        Self {
            cmds,
            expected_vol,
            expected_ntnl_vol,
        }
    }

    pub fn execute<B: OrderBook>(self) {
        let mut book = B::new_book();
        let Self {
            cmds,
            expected_vol,
            expected_ntnl_vol,
        } = self;
        for cmd in cmds {
            match cmd {
                Command::CancelOrder { id } => {
                    book.cancel_order(id);
                }
                Command::AddOrder {
                    id,
                    side,
                    size,
                    price,
                } => {
                    book.add_order(id, side, size, price);
                }
            }
        }
        assert_eq!(book.get_vol(), expected_vol);
        assert_eq!(book.get_ntnl_vol(), expected_ntnl_vol);
    }
}
