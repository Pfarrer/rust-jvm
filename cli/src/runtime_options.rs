use std::str::FromStr;

pub enum RuntimeOptions {
    Native,
}
impl FromStr for RuntimeOptions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "native" => Ok(RuntimeOptions::Native),
            _ => Err(format!("{} is not a valid runtime", s)),
        }
    }
}
