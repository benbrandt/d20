use r2d2::ManageConnection;
use rand::{Error, SeedableRng};
use rand_pcg::Pcg64;

#[derive(Default)]
pub struct RngConnectionManager;

impl RngConnectionManager {
    pub fn new() -> Self {
        Self
    }
}

impl ManageConnection for RngConnectionManager {
    type Connection = Pcg64;
    type Error = Error;

    fn connect(&self) -> Result<Pcg64, Error> {
        Ok(Pcg64::from_entropy())
    }

    fn is_valid(&self, _connection: &mut Pcg64) -> Result<(), Error> {
        Ok(())
    }

    fn has_broken(&self, _connection: &mut Pcg64) -> bool {
        false
    }
}
