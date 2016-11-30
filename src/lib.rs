#[macro_use]
extern crate log;
extern crate afterparty;

use afterparty::{Delivery, Hook};

// todo: fill in with config later
pub struct Transit {}

impl Hook for Transit {
    fn handle(&self, delivery: &Delivery) {
        info!("rec delivery {:#?}", delivery)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
