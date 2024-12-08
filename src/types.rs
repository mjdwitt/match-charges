use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::LazyLock;

use color_eyre::eyre::{eyre, Report, Result};
use num::Zero;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Order {
    name: String,
    value: u32, // in cents
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Charge {
    date: String,
    value: u32, // in cents
}

static ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(.*): (\d+).(\d\d)\s*$").unwrap());

fn parse(s: &str) -> Result<(String, u32)> {
    let re = &*ITEM;
    let (_, [id, dollars, cents]) = re.captures(s).ok_or(eyre!("invalid item {s}"))?.extract();
    Ok((
        id.to_string(),
        100u32 * dollars.parse::<u32>()? + cents.parse::<u32>()?,
    ))
}

fn write(f: &mut fmt::Formatter<'_>, id: &str, value: u32) -> fmt::Result {
    write!(f, "{}: {}.{:02}", id, value / 100, value % 100,)
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, &self.date, self.value)
    }
}

impl FromStr for Charge {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        parse(s).map(|(date, value)| Charge { date, value })
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, &self.name, self.value)
    }
}

impl FromStr for Order {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        parse(s).map(|(name, value)| Order { name, value })
    }
}

impl PartialEq<Charge> for Order {
    fn eq(&self, rhs: &Charge) -> bool {
        self.value == rhs.value
    }
}

impl PartialOrd<Charge> for Order {
    fn partial_cmp(&self, rhs: &Charge) -> Option<Ordering> {
        self.value.partial_cmp(&rhs.value)
    }
}

impl Sub<Charge> for Order {
    type Output = Self;
    fn sub(self, rhs: Charge) -> Self {
        Order {
            name: self.name,
            value: self.value - rhs.value,
        }
    }
}

impl Add<Order> for Order {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Order {
            name: self.name,
            value: self.value + rhs.value,
        }
    }
}

impl Zero for Order {
    fn zero() -> Self {
        Order {
            name: String::new(),
            value: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    fn set_zero(&mut self) {
        self.value = 0
    }
}
