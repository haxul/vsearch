use std::fmt::{Display, Formatter};

pub struct Vacancy {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area: Option<String>,
    pub(crate) salary: Option<Salary>
}

impl Display for Vacancy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let area = match &self.area {
            Some(s) => s,
            None => "unknown",
        };

        let salary = match &self.salary {
            Some(s) => s.to_string(),
            None => String::from("unknown"),
        };
        write!(f, "Vacancy(id: {}, name: {}, area: {}, salary: {}",
               &self.id, &self.name, area, salary)
    }
}

pub struct Salary {
    pub(crate) from: Option<i64>,
    pub(crate) to: Option<i64>,
    pub(crate) currency: String,
    pub(crate) is_gross: bool,
}

impl Display for Salary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let from = match &self.from {
            None => -1,
            Some(t) => *t
        };

        let to = match &self.to {
            None => -1,
            Some(t) => *t
        };
        write!(f, "from: {}, to: {}, currency: {}, is_gross: {})",
               from, to, &self.currency, &self.is_gross)
    }
}
