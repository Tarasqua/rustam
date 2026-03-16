pub trait Wallet {
    // INFO: Associated type - the type that will be used as the return type of the trait methods
    type Currency;

    fn amount(&self) -> Self::Currency;
    fn pay(&mut self, amount: Self::Currency);
}

pub struct CryptoWallet {
    balance: f64,
}

// INFO: Use the real type in implementation
impl Wallet for CryptoWallet {
    type Currency = f64; // Crypto can be fractional

    fn amount(&self) -> Self::Currency {
        self.balance
    }

    fn pay(&mut self, amount: Self::Currency) {
        self.balance -= amount;
        println!("Paid {}", amount);
    }
}

impl CryptoWallet {
    pub fn new(initial: f64) -> Self {
        Self { balance: initial }
    }
}

pub struct CashWallet {
    balance: u32,
}

impl Wallet for CashWallet {
    type Currency = u32; // Cash is always whole numbers

    fn amount(&self) -> Self::Currency {
        self.balance
    }

    fn pay(&mut self, amount: Self::Currency) {
        self.balance -= amount;
        println!("Paid {}", amount);
    }
}

pub fn print_balance<W>(wallet: &W)
where
    W: Wallet,
    W::Currency: std::fmt::Debug, // INFO: so that we can print it
{
    // INFO: we don't know the type directly, but Rust knows it's in Wallet
    let val = wallet.amount();
    println!("Something on account: {:?}", val);
}

pub fn make_wallet() -> Box<dyn Wallet<Currency = f64>> {
    Box::new(CryptoWallet::new(0.0))
}
