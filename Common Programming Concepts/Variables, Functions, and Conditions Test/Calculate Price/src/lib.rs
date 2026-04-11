pub fn calculate_price(amount: u32) -> u32 {
    if amount < 40 {
        amount * 2
    } else {
        amount
    }
}
