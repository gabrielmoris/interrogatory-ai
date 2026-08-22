use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SuspectId(u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FactId(u32);

impl SuspectId {
    pub fn new(id: u32) -> Self {
        SuspectId(id)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

/// Allows the possibility to display the number, Is a trait that I can't derive
/// so I have to create it.
impl fmt::Display for SuspectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "suspect #{}", self.0)
    }
}

/// Gives to suspectID the possibiklity to convert between types
impl From<u32> for SuspectId {
    fn from(id: u32) -> Self {
        SuspectId(id)
    }
}

impl FactId {
    pub fn new(id: u32) -> Self {
        FactId(id)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

/// Allows the possibility for FactId to display the number. Same as SuspectId
/// so I have to create it.
impl fmt::Display for FactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fact #{}", self.0)
    }
}

/// Gives to FactId the possibiklity to convert between types
impl From<u32> for FactId {
    fn from(id: u32) -> Self {
        FactId(id)
    }
}
