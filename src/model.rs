use input::{Record, Weekday as InputWeekday};
use std::convert::From;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Input {
    pub parents: Vec<Parent>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Parent {
    pub name: String,
    pub first_work_preferences: Vec<Weekday>,
    pub second_work_preferences: Vec<Weekday>,
    pub children: Vec<Child>,
    pub is_experienced: bool,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Child {
    first_caredfor_preferences: Vec<Weekday>,
    second_caredfor_preferences: Vec<Weekday>,
    days_in_school: u64,
}

impl From<Record> for Parent {
    fn from(record: Record) -> Parent {
        Parent {
            name: record.id.to_string(),
            first_work_preferences: vec![
                record.first_preference_parent_1,
                record.first_preference_parent_2,
                record.first_preference_parent_3,
            ].into_iter()
                .filter_map(|e| e.map(|v| v.into()))
                .collect(),
            second_work_preferences: vec![
                record.second_preference_parent_1,
                record.second_preference_parent_2,
                record.second_preference_parent_3,
            ].into_iter()
                .filter_map(|e| e.map(|v| v.into()))
                .collect(),
            is_experienced: record.experienced_parent.into(),
            children: {
                let mut ch = vec![Child {
                    first_caredfor_preferences: vec![
                        record.first_preference_first_child_1,
                        record.first_preference_first_child_2,
                        record.first_preference_first_child_3,
                        record.first_preference_first_child_4,
                        record.first_preference_first_child_5,
                    ].into_iter()
                        .filter_map(|e| e.map(|v| v.into()))
                        .collect(),
                    second_caredfor_preferences: vec![
                        record.second_preference_first_child_1,
                        record.second_preference_first_child_2,
                        record.second_preference_first_child_3,
                        record.second_preference_first_child_4,
                        record.second_preference_first_child_5,
                    ].into_iter()
                        .filter_map(|e| e.map(|v| v.into()))
                        .collect(),
                    days_in_school: record.days_for_first_child,
                }];
                if record.days_for_second_child.is_some() {
                    ch.push(Child {
                        first_caredfor_preferences: vec![
                            record.first_preference_second_child_1,
                            record.first_preference_second_child_2,
                            record.first_preference_second_child_3,
                            record.first_preference_second_child_4,
                            record.first_preference_second_child_5,
                        ].into_iter()
                            .filter_map(|e| e.map(|v| v.into()))
                            .collect(),
                        second_caredfor_preferences: vec![
                            record.second_preference_second_child_1,
                            record.second_preference_second_child_2,
                            record.second_preference_second_child_3,
                            record.second_preference_second_child_4,
                            record.second_preference_second_child_5,
                        ].into_iter()
                            .filter_map(|e| e.map(|v| v.into()))
                            .collect(),
                        days_in_school: record.days_for_second_child.unwrap(),
                    });
                }
                ch
            },
        }
    }
}

impl From<InputWeekday> for Weekday {
    fn from(wd: InputWeekday) -> Weekday {
        match wd {
            InputWeekday::Mon => Weekday::Monday,
            InputWeekday::Tues => Weekday::Tuesday,
            InputWeekday::Wed => Weekday::Wednesday,
            InputWeekday::Thurs => Weekday::Thursday,
            InputWeekday::Fri => Weekday::Friday,
        }
    }
}
