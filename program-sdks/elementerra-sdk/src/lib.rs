use std::fmt;

pub mod derive;
pub mod programs;

use programs::elementerra::state;

pub struct Player(state::Player);

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Player")
            .field("bump", &self.0.bump)
            .field("season_number", &self.0.season_number)
            .field("authority", &self.0.authority)
            .field(
                "number_of_elements_invented",
                &self.0.number_of_elements_invented,
            )
            .field(
                "number_of_failed_attempts",
                &self.0.number_of_failed_attempts,
            )
            .field(
                "number_of_elements_burned",
                &self.0.number_of_elements_burned,
            )
            .field(
                "number_of_elementum_burned",
                &self.0.number_of_elementum_burned,
            )
            // .field(
            //     "number_of_time_created_element",
            //     &self.0.number_of_time_created_element,
            // )
            // .field("referrer_treasury   ", &self.0.referrer_treasury)
            // .field("referrer_member", &self.0.referrer_member)
            .finish()
    }
}

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
