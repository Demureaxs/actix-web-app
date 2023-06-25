use serde::Serialize;

// super to access same directory modules
use super::super::enums::TaskStatus;
use super::base::Base;

#[derive(Serialize)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending { super_struct: base };
    }
}


