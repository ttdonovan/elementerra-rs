use std::fmt;

pub mod derive;
pub mod programs;

use programs::elementerra::state;

pub struct Season(state::Season);

impl fmt::Debug for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Season")
            .field("bump", &self.0.bump)
            .field("season_number", &self.0.season_number)
            .field("is_current_season", &self.0.is_current_season)
            .field(
                "elements_invented_currently_being_rewarded",
                &self.0.elements_invented_currently_being_rewarded,
            )
            .field(
                "total_elements_to_invent_available",
                &self.0.total_elements_to_invent_available,
            )
            .field("elements_invented", &self.0.elements_invented)
            .finish()
    }
}
