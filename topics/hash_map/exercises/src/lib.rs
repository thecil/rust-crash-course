use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut balance: HashMap<String, u32> = HashMap::new();
    balance.insert(address.to_string(), amount);
    balance
}
