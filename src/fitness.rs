use model::Input;
use model::Schedule;

const ANTIPREFERENCE_COST: f64 = 1000f64;
const NOT_FIRST_PREFERENCE_COST: f64 = 10f64;
const NOT_SECOND_PREFERENCE_COST: f64 = 5f64;

pub fn compute_fitness(input: &Input, schedule: &Schedule) -> f64 {
    let mut preference_cost = 0.0;
    for day in vec![
        &schedule.monday,
        &schedule.tuesday,
        &schedule.wednesday,
        &schedule.thursday,
        &schedule.friday,
    ] {
        for parent in &day.parents {
            for anti_preference in &parent.anti_preferences {
                if anti_preference == &day.weekday {
                    preference_cost = preference_cost + ANTIPREFERENCE_COST;
                }
            }
            
            if !parent.first_work_preferences.contains(&day.weekday) {
                preference_cost = preference_cost + NOT_FIRST_PREFERENCE_COST;
            }

            if !parent.second_work_preferences.contains(&day.weekday) {
                preference_cost = preference_cost + NOT_FIRST_PREFERENCE_COST;
            }
        }
    }
    preference_cost
}
