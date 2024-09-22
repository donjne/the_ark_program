pub const MAX_NAME_LENGTH: usize = 50;
pub const MAX_DESCRIPTION_LENGTH: usize = 200;
pub const MAX_TITLE_LENGTH: usize = 100;

pub const MAX_ASSEMBLIES: usize = 10;
pub const MAX_POLICY_AREAS: usize = 20;
pub const MAX_ASSEMBLY_MEMBERS: usize = 100;
pub const MAX_POLICY_AREAS_PER_ASSEMBLY: usize = 5;
pub const MAX_ASSEMBLIES_PER_POLICY_AREA: usize = 5;
pub const MAX_PROPOSALS_PER_POLICY_AREA: usize = 50;
pub const MAX_TREASURIES: usize = 10;

pub const VOTING_PERIOD: i64 = 7 * 24 * 60 * 60; // 7 days in seconds