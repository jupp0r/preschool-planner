#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Input {
    pub parents: Vec<Parent>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Parent {
    pub name: String,
    pub first_work_preferences: Vec<WeekDay>,
    pub second_work_preferences: Vec<WeekDay>,
    pub children: Vec<Child>,
    pub is_inexperienced: bool,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Child {
    first_caredfor_preferences: Vec<WeekDay>,
    second_caredfor_preferences: Vec<WeekDay>,
    age: u64,
}
