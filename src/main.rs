mod coingecko;
mod yahoo;

fn main() {
    coingecko::get_coingecko();
    yahoo::get_yahoo();
    coingecko::gecko_get_price("bitcoin".to_string());
}
