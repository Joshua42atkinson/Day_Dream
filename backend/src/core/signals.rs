use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalCode {
    Blue,  // Feed-Up: Where am I going? (Destination Confirmed)
    Amber, // Feed-Back: How am I going? (Status Report)
    Green, // Feed-Forward: Where to next? (Proceed to Next Sector)
    Red,   // Stop: Safety Lockout / Derailment Risk
}

impl SignalCode {
    pub fn message(&self) -> &'static str {
        match self {
            SignalCode::Blue => "Destination Confirmed.",
            SignalCode::Amber => "Status Report Received.",
            SignalCode::Green => "Proceed to Next Sector.",
            SignalCode::Red => "Safety Stop: Assistance Required.",
        }
    }
}
