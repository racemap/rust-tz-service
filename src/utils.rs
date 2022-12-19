pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxUnitResult = std::result::Result<(), BoxError>;

#[allow(non_upper_case_globals)]
pub const UnitOk: BoxUnitResult = Ok(());
