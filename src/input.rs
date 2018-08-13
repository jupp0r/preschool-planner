use failure::Error;

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
pub struct Record {
    pub id: u64,
    #[serde(rename = "first preference parent 1")]
    pub first_preference_parent_1: Option<Weekday>,
    #[serde(rename = "first preference parent 2")]
    pub first_preference_parent_2: Option<Weekday>,
    #[serde(rename = "first preference parent 3")]
    pub first_preference_parent_3: Option<Weekday>,
    #[serde(rename = "first preference parent 4")]
    pub first_preference_parent_4: Option<Weekday>,
    #[serde(rename = "first preference parent 5")]
    pub first_preference_parent_5: Option<Weekday>,
    #[serde(rename = "second preference parent 1")]
    pub second_preference_parent_1: Option<Weekday>,
    #[serde(rename = "second preference parent 2")]
    pub second_preference_parent_2: Option<Weekday>,
    #[serde(rename = "second preference parent 3")]
    pub second_preference_parent_3: Option<Weekday>,
    #[serde(rename = "anti preference parent")]
    pub anti_preference_parent: Option<Weekday>,
    #[serde(rename = "experienced parent")]
    pub experienced_parent: YesNo,
    #[serde(rename = "baby room")]
    pub baby_room: YesNo,
    //    #[serde(rename = "introvert/extrovert")]
    //    pub introvert_extrovert: IntrovertExtrovert,
    #[serde(rename = "days for first child")]
    pub days_for_first_child: u64,
    #[serde(rename = "first preference first child 1")]
    pub first_preference_first_child_1: Option<Weekday>,
    #[serde(rename = "first preference first child 2")]
    pub first_preference_first_child_2: Option<Weekday>,
    #[serde(rename = "first preference first child 3")]
    pub first_preference_first_child_3: Option<Weekday>,
    #[serde(rename = "first preference first child 4")]
    pub first_preference_first_child_4: Option<Weekday>,
    #[serde(rename = "first preference first child 5")]
    pub first_preference_first_child_5: Option<Weekday>,
    #[serde(rename = "first preference second child 1")]
    pub first_preference_second_child_1: Option<Weekday>,
    #[serde(rename = "first preference second child 2")]
    pub first_preference_second_child_2: Option<Weekday>,
    #[serde(rename = "first preference second child 3")]
    pub first_preference_second_child_3: Option<Weekday>,
    #[serde(rename = "first preference second child 4")]
    pub first_preference_second_child_4: Option<Weekday>,
    #[serde(rename = "first preference second child 5")]
    pub first_preference_second_child_5: Option<Weekday>,

    #[serde(rename = "days for second child")]
    pub days_for_second_child: Option<u64>,
    #[serde(rename = "second preference first child 1")]
    pub second_preference_first_child_1: Option<Weekday>,
    #[serde(rename = "second preference first child 2")]
    pub second_preference_first_child_2: Option<Weekday>,
    #[serde(rename = "second preference first child 3")]
    pub second_preference_first_child_3: Option<Weekday>,
    #[serde(rename = "second preference first child 4")]
    pub second_preference_first_child_4: Option<Weekday>,
    #[serde(rename = "second preference first child 5")]
    pub second_preference_first_child_5: Option<Weekday>,
    #[serde(rename = "second preference second child 1")]
    pub second_preference_second_child_1: Option<Weekday>,
    #[serde(rename = "second preference second child 2")]
    pub second_preference_second_child_2: Option<Weekday>,
    #[serde(rename = "second preference second child 3")]
    pub second_preference_second_child_3: Option<Weekday>,
    #[serde(rename = "second preference second child 4")]
    pub second_preference_second_child_4: Option<Weekday>,
    #[serde(rename = "second preference second child 5")]
    pub second_preference_second_child_5: Option<Weekday>,
}

pub fn load_from_file(path: &str) -> Result<Vec<Record>, Error> {
    use csv;
    let mut reader = csv::Reader::from_path(&path)?;
    reader
        .deserialize()
        .map(|r| r.map_err(|e| e.into()))
        .collect()
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Weekday {
    Mon,
    Tues,
    Wed,
    Thurs,
    Fri,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    Yes,
    No,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntrovertExtrovert {
    Introvert,
    Extrovert,
}

impl From<YesNo> for bool {
    fn from(i: YesNo) -> bool {
        match i {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = load_from_file("./enrollment.csv");
        assert!(res.is_ok(), "error: {:?}", res)
    }
}
