pub struct Variable {}

impl core::fmt::Debug for Variable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Variable").finish()
    }
}
