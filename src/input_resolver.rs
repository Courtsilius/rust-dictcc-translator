pub struct InputResolver {
    value: Vec<String>,
    status: i32,
}

impl InputResolver {
    pub fn value(&self) -> &Vec<String> {
        &self.value
    }
    pub fn status(&self) -> &i32 {
        &self.status
    }

    pub fn new(value: Vec<String>, status: i32) -> InputResolver {
        InputResolver { value, status }
    }
}
