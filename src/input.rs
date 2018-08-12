use failure::Error;

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
pub struct Record {
    pub id: u64,
    pub first_preference_parent_1: Option<Weekday>,
    pub first_preference_parent_2: Option<Weekday>,
    pub first_preference_parent_3: Option<Weekday>,
    pub second_preference_parent_1: Option<Weekday>,
    pub second_preference_parent_2: Option<Weekday>,
    pub second_preference_parent_3: Option<Weekday>,
    pub anti_preference_parent: Option<Weekday>,
    pub experienced_parent: YesNo,
    pub baby_room: YesNo,
    //    pub introvert_extrovert: IntrovertExtrovert,
    pub days_for_first_child: u64,
    pub first_preference_first_child_1: Option<Weekday>,
    pub first_preference_first_child_2: Option<Weekday>,
    pub first_preference_first_child_3: Option<Weekday>,
    pub first_preference_first_child_4: Option<Weekday>,
    pub first_preference_first_child_5: Option<Weekday>,
    pub first_preference_second_child_1: Option<Weekday>,
    pub first_preference_second_child_2: Option<Weekday>,
    pub first_preference_second_child_3: Option<Weekday>,
    pub first_preference_second_child_4: Option<Weekday>,
    pub first_preference_second_child_5: Option<Weekday>,

    pub second_preference_first_child_1: Option<Weekday>,
    pub second_preference_first_child_2: Option<Weekday>,
    pub second_preference_first_child_3: Option<Weekday>,
    pub second_preference_first_child_4: Option<Weekday>,
    pub second_preference_first_child_5: Option<Weekday>,
    pub second_preference_second_child_1: Option<Weekday>,
    pub second_preference_second_child_2: Option<Weekday>,
    pub second_preference_second_child_3: Option<Weekday>,
    pub second_preference_second_child_4: Option<Weekday>,
    pub second_preference_second_child_5: Option<Weekday>,
}

pub fn load_from_file(path: &str) -> Result<Vec<Record>, Error> {
    use csv;
    let mut reader = csv::Reader::from_path(&path)?;
    Ok(reader.deserialize().map(|r| r.unwrap()).collect())
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
pub enum Weekday {
    Mon,
    Tues,
    Wed,
    Thurs,
    Fri,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
pub enum YesNo {
    Yes,
    No,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
pub enum IntrovertExtrovert {
    Introvert,
    Extrovert,
}
