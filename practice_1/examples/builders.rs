use std::fmt;

fn main() {
    let tx = Transaction {
        sender: "Alice".to_string(),
        receiver: "".to_string(),
        amount: 100.,
        currency: "".to_string(),
        timestamp: 123456789,
        hash: "123456789".to_string(),
        signature: "123456789".to_string(),
        nonce: 0,
        status: "pending".to_string(),
        fee: 0.0,
    };

    let builder_tx = Transaction::builder()
        .sender("Alice".to_string())
        .amount(100.)
        .timestamp(123456789)
        .build("private_key");
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub currency: String,
    pub timestamp: u64,
    pub hash: String,
    pub signature: String,
    pub nonce: u64,
    pub status: String,
    pub fee: f64,
}

impl Transaction {
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::default()
    }
}

#[derive(Debug, Default)]
struct TransactionBuilder {
    sender: String,
    receiver: String,
    amount: f64,
    currency: String,
    timestamp: u64,
    hash: String,
    signature: String,
    nonce: u64,
    status: String,
    fee: f64,
}

impl TransactionBuilder {
    pub fn sender(mut self, sender: String) -> Self {
        self.sender = sender;
        self
    }

    pub fn receiver(mut self, receiver: String) -> Self {
        self.receiver = receiver;
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn currency(mut self, currency: String) -> Self {
        self.currency = currency;
        self
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = nonce;
        self
    }

    pub fn status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    pub fn fee(mut self, fee: f64) -> Self {
        self.fee = fee;
        self
    }

    pub fn build(self, private_key: &str) -> Transaction {
        let hash = self.calculate_hash();
        let signature = self.sign(hash.as_str(), private_key);

        Transaction {
            sender: self.sender,
            receiver: self.receiver,
            amount: self.amount,
            currency: self.currency,
            timestamp: self.timestamp,
            hash,
            signature,
            nonce: self.nonce,
            status: self.status,
            fee: self.fee,
        }
    }

    fn calculate_hash(&self) -> String {
        // TODO: implement hash caclulation
        String::from("hash")
    }

    fn sign(&self, _hash: &str, _private_key: &str) -> String {
        // TODO: implement signature calculation
        String::from("signature")
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {}: {} {}",
            self.sender, self.receiver, self.amount, self.currency
        )
    }
}
