pub struct TimeInterval {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl TimeInterval {
    pub fn from_seconds(total_seconds: u64) -> Self {
        let mut n = total_seconds;
        let days = n / 86400;

        n %= 86400;
        let hours = n / 3600;

        n %= 3600;
        let minutes = n / 60;

        n %= 60;
        let seconds = n;

        TimeInterval {
            days,
            hours,
            minutes,
            seconds,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{}{}{}{}",
            Self::format_component(self.days, "day"),
            Self::format_component(self.hours, "hour"),
            Self::format_component(self.minutes, "minute"),
            Self::format_component(self.seconds, "second"),
        )
    }

    fn format_component(value: u64, singular_name: &str) -> String {
        if value > 0 {
            if value > 1 {
                format!("{} {}s ", value, singular_name)
            } else {
                format!("{} {} ", value, singular_name)
            }
        } else {
            "".to_owned()
        }
    }
}
