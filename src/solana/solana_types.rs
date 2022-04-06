#[derive(Clone, Debug)]
pub struct Wallet {
    address: String,
    alt_name: String,
    tokens: Vec<Token>,
    transactions: Vec<Transaction>,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub(crate) signature: String,
    pub(crate) timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct Token {
    address: String,
    alt_name: u64,
    tag: Tag,
}

#[derive(Clone, Debug)]
pub enum Tag {
    Stablecoin,
    Normal,
    Lp
}