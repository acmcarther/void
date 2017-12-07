//! Allan provides variance and deviation tools for stability analysis
//!
//! # Goals
//! * provide streaming variance and deviations from series data
//! * pre-allocated datastructures
//!
//! # Usage
//!
//! Create a new instance, add records, retrieve statistic
//!
//! # Example
//!
//! Create a new instance, add a few records, retrieve allan deviation
//!
//! ```
//!
//! use allan::*;
//!
//! // a default Allan
//! let mut allan = Allan::new();
//! for _ in 0..100 {
//!     allan.record(1.0);
//! }
//! assert_eq!(allan.get(1).unwrap().deviation().unwrap(), 0.0);
//!
//! // a configured Allan
//! let mut allan = Allan::configure().max_tau(10_000).build().unwrap();

use std::collections::VecDeque;

/// the main datastructure for Allan
#[derive(Clone)]
pub struct Allan {
    samples: usize,
    config: Config,
    taus: Vec<Tau>,
    buffer: VecDeque<f64>,
}

/// a duration-based bucket for the stability metric
#[derive(Copy, Clone)]
pub struct Tau {
    value: f64,
    count: u64,
    tau: usize,
}

impl Tau {
    // construct a new `Tau`
    fn new(tau: usize) -> Tau {
        Tau {
            value: 0.0_f64,
            count: 0_u64,
            tau: tau,
        }
    }

    /// returns the time value of the `Tau`
    pub fn tau(self) -> usize {
        self.tau
    }

    // add a value to the `Tau`
    fn add(&mut self, value: f64) {
        self.value += value;
        self.count += 1;
    }

    /// returns the count of samples at `Tau`
    pub fn count(self) -> u64 {
        self.count
    }

    // return the sum at `Tau`
    pub fn value(self) -> f64 {
        self.value
    }

    /// returns the Allan Variance at `Tau`
    pub fn variance(self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        Some(
            self.value() / (2.0_f64 * self.count() as f64) / self.tau() as f64,
        )
    }

    /// returns the Allan Deviation at `Tau`
    pub fn deviation(self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        Some(
            (self.value() / (2.0_f64 * self.count() as f64)).powf(0.5) / self.tau() as f64,
        )
    }
}

/// describes the gaps between `Tau` and impacts space and computational costs
#[derive(Copy, Clone)]
pub enum Style {
    SingleTau(usize), // single specified Tau
    AllTau, // all Tau from 1 ... Tau (inclusive)
    Decade, // 1,10,100, ... Tau (inclusive)
    DecadeDeci, // 1, 2, 3, .., 9, 10, 20, 30, .. Tau (inclusive)
    Decade124, // 1, 2, 4, 10, 20, 40, ... Tau (inclusive)
    Decade1248, // 1, 2, 4, 8, 10, 20, 40, ... Tau (inclusive)
    Decade125, // 1, 2, 5, 10, 20, 50, ... Tau (inclusive)
}

/// used to configure an `Allan`
#[derive(Copy, Clone)]
pub struct Config {
    max_tau: usize,
    style: Style,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_tau: 1_000,
            style: Style::DecadeDeci,
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn max_tau(mut self, max_tau: usize) -> Self {
        self.max_tau = max_tau;
        self
    }

    pub fn build(self) -> Option<Allan> {
        Allan::configured(self)
    }
}

impl Default for Allan {
    fn default() -> Allan {
        Config::default().build().unwrap()
    }
}

impl Allan {
    /// create a new Allan
    pub fn new() -> Allan {
        Default::default()
    }

    fn decade_tau(max: usize, steps: Vec<usize>) -> Vec<Tau> {
        let mut p = 0;
        let mut t = 1;
        let mut taus: Vec<Tau> = Vec::new();

        while t <= max {
            for i in &steps {
                t = i * 10_u32.pow(p) as usize;
                if t <= max {
                    taus.push(Tau::new(t));
                }
            }
            p += 1;
        }
        taus
    }

    pub fn configure() -> Config {
        Config::default()
    }

    fn configured(config: Config) -> Option<Allan> {
        let samples = config.max_tau * 2 + 1; // this will vary by type

        let buffer = VecDeque::with_capacity(samples as usize);

        let mut taus: Vec<Tau> = Vec::new();

        match config.style {
            Style::SingleTau(t) => {
                taus.push(Tau::new(t));
            }
            Style::AllTau => {
                for t in 1..(config.max_tau + 1) {
                    taus.push(Tau::new(t));
                }
            }
            Style::Decade125 => taus = Allan::decade_tau(config.max_tau, vec![1, 2, 5]),
            Style::Decade124 => taus = Allan::decade_tau(config.max_tau, vec![1, 2, 4]),
            Style::Decade1248 => taus = Allan::decade_tau(config.max_tau, vec![1, 2, 4, 8]),
            Style::DecadeDeci => {
                taus = Allan::decade_tau(config.max_tau, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
            }
            Style::Decade => taus = Allan::decade_tau(config.max_tau, vec![1]),
        }

        Some(Allan {
            buffer: buffer,
            config: config,
            samples: samples,
            taus: taus,
        })
    }

    /// add a record
    pub fn record(&mut self, value: f64) {
        self.buffer.push_front(value);
        self.calculate();
        if self.buffer.len() == self.samples {
            let _ = self.buffer.pop_back();
        }
    }

    // recalculate values
    fn calculate(&mut self) {
        for tau in &mut self.taus {
            let t = tau.tau() as usize;
            if (2 * t) < self.buffer.len() {
                let var: f64 = self.buffer[(2 * t)] - 2.0_f64 * self.buffer[t] + self.buffer[0];
                tau.add(var.powf(2.0_f64));
            }
        }
    }

    /// print deviations for all `Tau`
    pub fn print(&self) {
        for tau in &self.taus {
            if tau.count() >= 3 {
                println!("{} {}", tau.variance().unwrap_or(0.0), tau.tau());
            } else {
                println!("0.0 {}", tau.tau())
            }
        }
    }

    /// get a single `Tau` from the `Allan`
    pub fn get(&self, tau: usize) -> Option<Tau> {
        if tau > self.config.max_tau {
            return None;
        }
        for t in &self.taus {
            if t.tau() == tau {
                return Some(*t);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate probability;
    extern crate rand;

    use self::probability::prelude::*;
    use self::rand::distributions::{IndependentSample, Range};
    use super::*;

    #[test]
    fn white_noise() {
        let mut allan = Allan::configure()
            .max_tau(1000)
            .style(Style::AllTau)
            .build()
            .unwrap();
        let mut rng = rand::thread_rng();
        let between = Range::new(0.0, 1.0);
        for _ in 0..10_000 {
            let v = between.ind_sample(&mut rng);
            allan.record(v);
        }
        for t in 1..1000 {
            let v = allan
                .get(t)
                .unwrap_or_else(|| {
                    print!("error fetching for tau: {}", t);
                    panic!("error")
                })
                .deviation()
                .unwrap() * t as f64;
            if v <= 0.4 || v >= 0.6 {
                panic!("tau: {} value: {} outside of range", t, v);
            }
        }
    }

    #[test]
    fn pink_noise() {
        let mut allan = Allan::configure()
            .max_tau(1000)
            .style(Style::AllTau)
            .build()
            .unwrap();

        let mut source = source::default();
        let distribution = Beta::new(1.0, 3.0, 0.0, 1.0);

        for _ in 0..10_000 {
            let v = distribution.sample(&mut source);
            allan.record(v);
        }
        for t in 1..1000 {
            let v = allan
                .get(t)
                .unwrap_or_else(|| {
                    println!("error fetching for tau: {}", t);
                    panic!("error")
                })
                .deviation()
                .unwrap() * t as f64 * 0.5;
            if v <= 0.1 || v >= 0.3 {
                panic!("tau: {} value: {} outside of range", t, v);
            }
        }
    }
}
