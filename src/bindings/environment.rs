#[repr(u32)]
pub enum Environment {
    Sandbox,
    Production
}

impl From<openlimits_exchange::exchange::Environment> for Environment {
    fn from(from: openlimits_exchange::exchange::Environment) -> Self {
        match from {
            openlimits_exchange::exchange::Environment::Sandbox => Environment::Sandbox,
            openlimits_exchange::exchange::Environment::Production => Environment::Production
        }
    }
}

impl From<Environment> for openlimits_exchange::exchange::Environment {
    fn from(from: Environment) -> Self {
        match from {
            Environment::Sandbox => openlimits_exchange::exchange::Environment::Sandbox,
            Environment::Production => openlimits_exchange::exchange::Environment::Production
        }
    }
}