use chrono::TimeDelta;
use jellyfin_api::types::BaseItemDto;

use crate::{tr, utils::ticks::ticks_to_seconds};

pub(crate) trait RunTime {
    fn run_time(&self) -> Option<String>;
}

impl RunTime for BaseItemDto {
    fn run_time(&self) -> Option<String> {
        let Some(run_time_ticks) = self.run_time_ticks else {
            return None;
        };

        let run_time = TimeDelta::seconds(ticks_to_seconds(run_time_ticks) as i64);
        let hours = run_time.num_hours();
        let minutes = run_time.num_minutes() - (60 * hours);

        Some(
            tr!("media-details-run-time", {
                "hours" => hours,
                "minutes" => minutes,
            })
            .to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ticks::seconds_to_ticks;

    use super::*;

    #[test]
    fn test_run_time_hours_minutes() -> Result<(), String> {
        let item: BaseItemDto = BaseItemDto::builder()
            .run_time_ticks(Some(seconds_to_ticks(8088) as i64))
            .try_into()?;
        assert_eq!(Some("2h 14m"), item.run_time().as_deref());
        Ok(())
    }

    #[test]
    fn test_run_time_minutes() -> Result<(), String> {
        let item: BaseItemDto = BaseItemDto::builder()
            .run_time_ticks(Some(seconds_to_ticks(420) as i64))
            .try_into()?;
        assert_eq!(Some("7m"), item.run_time().as_deref());
        Ok(())
    }
}
