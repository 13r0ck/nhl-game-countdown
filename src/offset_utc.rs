use chrono::FixedOffset;

#[derive(Default, Clone, Copy, Debug)]
pub enum OffsetUTC {
    Minus1200,
    Minus1100,
    Minus1000,
    Minus0930,
    Minus0900,
    Minus0800,
    #[default]
    Minus0700,
    Minus0600,
    Minus0500,
    Minus0400,
    Minus0300,
    Minus0330,
    Minus0200,
    Minus0100,
    Utc,
    Plus0100,
    Plus0200,
    Plus0300,
    Plus0330,
    Plus0400,
    Plus0430,
    Plus0500,
    Plus0530,
    Plus0545,
    Plus0600,
    Plus0700,
    Plus0800,
    Plus0900,
    Plus0930,
    Plus1000,
    Plus1030,
    Plus1100,
    Plus1200,
    Plus1245,
    Plus1300,
    Plus1400,
}

impl OffsetUTC {
    pub fn new(offset: &str) -> Self {
        match offset {
            "UTC-1200" => OffsetUTC::Minus1200,
            "UTC-1100" => OffsetUTC::Minus1100,
            "UTC-1000" => OffsetUTC::Minus1000,
            "UTC-0930" => OffsetUTC::Minus0930,
            "UTC-0900" => OffsetUTC::Minus0900,
            "UTC-0800" => OffsetUTC::Minus0800,
            "UTC-0700" => OffsetUTC::Minus0700,
            "UTC-0600" => OffsetUTC::Minus0600,
            "UTC-0500" => OffsetUTC::Minus0500,
            "UTC-0400" => OffsetUTC::Minus0400,
            "UTC-0300" => OffsetUTC::Minus0300,
            "UTC-0330" => OffsetUTC::Minus0330,
            "UTC-0200" => OffsetUTC::Minus0200,
            "UTC-0100" => OffsetUTC::Minus0100,
            "UTC-0000" => OffsetUTC::Utc,
            "UTC+0100" => OffsetUTC::Plus0100,
            "UTC+0200" => OffsetUTC::Plus0200,
            "UTC+0300" => OffsetUTC::Plus0300,
            "UTC+0330" => OffsetUTC::Plus0330,
            "UTC+0400" => OffsetUTC::Plus0400,
            "UTC+0430" => OffsetUTC::Plus0430,
            "UTC+0500" => OffsetUTC::Plus0500,
            "UTC+0530" => OffsetUTC::Plus0530,
            "UTC+0545" => OffsetUTC::Plus0545,
            "UTC+0600" => OffsetUTC::Plus0600,
            "UTC+0700" => OffsetUTC::Plus0700,
            "UTC+0800" => OffsetUTC::Plus0800,
            "UTC+0900" => OffsetUTC::Plus0900,
            "UTC+0930" => OffsetUTC::Plus0930,
            "UTC+1000" => OffsetUTC::Plus1000,
            "UTC+1030" => OffsetUTC::Plus1030,
            "UTC+1100" => OffsetUTC::Plus1100,
            "UTC+1200" => OffsetUTC::Plus1200,
            "UTC+1245" => OffsetUTC::Plus1245,
            "UTC+1300" => OffsetUTC::Plus1300,
            "UTC+1400" => OffsetUTC::Plus1400,
            _ => OffsetUTC::default(),
        }
    }
}

impl From<OffsetUTC> for FixedOffset {
    fn from(offset: OffsetUTC) -> FixedOffset {
        const HOURS: i32 = 3600;
        const MINS: i32 = 60;
        const EAST: fn(i32) -> Option<FixedOffset> = FixedOffset::east_opt;
        const WEST: fn(i32) -> Option<FixedOffset> = FixedOffset::west_opt;
        match offset {
            OffsetUTC::Minus1200 => WEST(HOURS * 12),
            OffsetUTC::Minus1100 => WEST(HOURS * 11),
            OffsetUTC::Minus1000 => WEST(HOURS * 10),
            OffsetUTC::Minus0930 => WEST(HOURS * 9 + MINS * 30),
            OffsetUTC::Minus0900 => WEST(HOURS * 9),
            OffsetUTC::Minus0800 => WEST(HOURS * 8),
            OffsetUTC::Minus0700 => WEST(HOURS * 7),
            OffsetUTC::Minus0600 => WEST(HOURS * 6),
            OffsetUTC::Minus0500 => WEST(HOURS * 5),
            OffsetUTC::Minus0400 => WEST(HOURS * 4),
            OffsetUTC::Minus0300 => WEST(HOURS * 3),
            OffsetUTC::Minus0330 => WEST(HOURS * 3 + MINS * 30),
            OffsetUTC::Minus0200 => WEST(HOURS * 2),
            OffsetUTC::Minus0100 => WEST(HOURS),
            OffsetUTC::Utc => WEST(0),
            OffsetUTC::Plus0100 => EAST(HOURS),
            OffsetUTC::Plus0200 => EAST(HOURS * 2),
            OffsetUTC::Plus0300 => EAST(HOURS * 3),
            OffsetUTC::Plus0330 => EAST(HOURS * 3 + MINS * 30),
            OffsetUTC::Plus0400 => EAST(HOURS * 4),
            OffsetUTC::Plus0430 => EAST(HOURS * 4 + MINS * 30),
            OffsetUTC::Plus0500 => EAST(HOURS * 5),
            OffsetUTC::Plus0530 => EAST(HOURS * 5 + MINS * 30),
            OffsetUTC::Plus0545 => EAST(HOURS * 5 + MINS * 45),
            OffsetUTC::Plus0600 => EAST(HOURS * 6),
            OffsetUTC::Plus0700 => EAST(HOURS * 7),
            OffsetUTC::Plus0800 => EAST(HOURS * 8),
            OffsetUTC::Plus0900 => EAST(HOURS * 9),
            OffsetUTC::Plus0930 => EAST(HOURS * 9 + MINS * 30),
            OffsetUTC::Plus1000 => EAST(HOURS * 10),
            OffsetUTC::Plus1030 => EAST(HOURS * 10 + MINS * 30),
            OffsetUTC::Plus1100 => EAST(HOURS * 11),
            OffsetUTC::Plus1200 => EAST(HOURS * 12),
            OffsetUTC::Plus1245 => EAST(HOURS * 12 + MINS * 45),
            OffsetUTC::Plus1300 => EAST(HOURS * 13),
            OffsetUTC::Plus1400 => EAST(HOURS * 14),
        }
        .unwrap()
    }
}

impl chrono::offset::Offset for OffsetUTC {
    fn fix(&self) -> FixedOffset {
        (*self).into()
    }
}
