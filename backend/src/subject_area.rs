pub use smallvec::{smallvec, SmallVec};

// TODO auto generate subject areas and admin_of method from config file

#[derive(Copy, Clone, Debug, Hash)]
pub enum SubjectArea {
    Fencing,
    VisualArts,
    IndustrialArts,
    CrossCountry,
}

impl SubjectArea {
    pub fn admin_of<S>(id: S) -> SmallVec<[SubjectArea; 1]>
    where
        S: AsRef<str>,
    {
        match id.as_ref() {
            "400" => smallvec![Self::Fencing],
            "401" => smallvec![Self::VisualArts],
            "402" => smallvec![Self::IndustrialArts, Self::CrossCountry],
            _ => smallvec![],
        }
    }
}
