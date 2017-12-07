extern crate chrono;
#[macro_use]
extern crate monitoring;
#[macro_use]
extern crate lazy_static;

use monitoring::Metric;
use monitoring::MetricEntry;
use monitoring::LabelValue;
use std::collections::VecDeque;

fn now_micros() -> u64 {
  let epoch_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
  // Technically this can overflow, but it wont for over 200_000 years.
  // ... so I'm really not going to bother with this error handling.
  chrono::Duration::from_std(epoch_time)
    .unwrap()
    .num_microseconds()
    .unwrap() as u64
}

lazy_static! {
  pub static ref SIMPLE_TEST_METRIC: Metric = {
    Metric::new_metric(
      "/monitoring/test/simple_test_metric",
      vec!["foo"],
      defined_here!())
  };
}

#[test]
fn test_metric_is_accessible() {
  assert!(SIMPLE_TEST_METRIC.peek_records().is_empty());
  SIMPLE_TEST_METRIC.set_value(MetricEntry {
    data: 100.0,
    labels: vec![LabelValue {
      name: "foo".to_owned(),
      value: "bar".to_owned(),
    }],
    timestamp_micros: now_micros()
  });
  assert!(!SIMPLE_TEST_METRIC.peek_records().is_empty());
}
