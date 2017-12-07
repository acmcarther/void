#![feature(used)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate zcfg;

extern crate futures;
extern crate hyper;
extern crate radix_trie;

use radix_trie::Trie;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;
use std::sync::Arc;

// TODO(acmcarther): Update zcfg to provide default impl for u16
define_cfg!(monitoring_daemon_port,
            u32,
            4321u32,
            "What port to run the monitoring daemon on");
define_pub_cfg!(maximum_cached_metric_records_per_metric,
            u32,
            1000u32,
            "How many metric records to retain before spilling some.");
define_pub_cfg!(record_count_to_spill_when_overfull,
            u32,
            100u32,
            "How many records to spill from a metric when it gets too full (unscraped too long)");

lazy_static! {
  pub static ref METRIC_REGISTRY: RwLock<Trie<&'static str, MetricRecord>> = {
    RwLock::new(Trie::new())
  };
}

pub fn init() {
  
}

pub struct MetricRecord {
  name: &'static str,
  labels: Vec<&'static str>,
  definition_site: DefinitionSite,
  data: Arc<RwLock<MetricData>>
}

impl MetricRecord {
  fn install(name: &'static str,
         labels: Vec<&'static str>,
         definition_site: DefinitionSite,
         data: Arc<RwLock<MetricData>>) {
    let decl = MetricRecord {
      name: name,
      labels: labels,
      definition_site: definition_site,
      data: data,
    };
    METRIC_REGISTRY.deref().write().unwrap().insert(name, decl);
  }

  pub fn take_records(&self) -> VecDeque<MetricEntry> {
    let mut records = VecDeque::new();
    std::mem::swap(&mut records, &mut self.data.write().unwrap().in_memory_records);
    records
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LabelValue {
  pub name: String,
  pub value: String,
}

pub struct Metric {
  name: &'static str,
  labels: Vec<&'static str>,
  definition_site: DefinitionSite,
  data: Arc<RwLock<MetricData>>
}

impl Metric {
  pub fn new_metric(name: &'static str,
                    labels: Vec<&'static str>,
                    definition_site: DefinitionSite) -> Metric {
    let underlying_data = Arc::new(RwLock::new(MetricData::new(name)));

    // Write a "declaration" into the global registry
    MetricRecord::install(name, labels.clone(), definition_site.clone(), underlying_data.clone());

    // ... But produce a metric for the user to retain
    Metric {
      name: name,
      labels: labels,
      definition_site: definition_site,
      data: underlying_data,
    }
  }

  pub fn peek_records(&self) -> VecDeque<MetricEntry> {
    self.data.read().unwrap().in_memory_records.clone()
  }

  pub fn set_value(&self, entry: MetricEntry) {
    let mut locked_data = self.data.write().unwrap();
    locked_data.add_value(entry);
  }
}

pub struct MetricData {
  name: &'static str,
  in_memory_records: VecDeque<MetricEntry>,
  latest_values: HashMap<Vec<LabelValue>, MetricEntry>,
  max_records_to_cache: u32,
  count_records_to_spill_when_full: u32
}

impl MetricData {
  pub fn new(name: &'static str) -> MetricData {
    MetricData {
      name: name,
      in_memory_records: VecDeque::new(),
      latest_values: HashMap::new(),
      max_records_to_cache: maximum_cached_metric_records_per_metric::CONFIG.get_value(),
      count_records_to_spill_when_full: record_count_to_spill_when_overfull::CONFIG.get_value(),
    }
  }

  pub fn add_value(&mut self, entry: MetricEntry) {
    self.latest_values.insert(entry.labels.clone(), entry.clone());
    if self.in_memory_records.len() > self.max_records_to_cache as usize {
      warn!("Metric \"{}\" got too full (over \"{}\" total records)! Configure your scraper to scrape more frequently, \
            or your service to log less frequently. Spilling {} events.",
            self.name,
            self.max_records_to_cache,
            self.count_records_to_spill_when_full);
      for _ in 0..self.count_records_to_spill_when_full {
        self.in_memory_records.pop_front();
      }

    }
    self.in_memory_records.push_back(entry);
  }
}

#[derive(Clone)]
pub struct MetricEntry {
  pub data: f64,
  pub labels: Vec<LabelValue>,
  pub timestamp_micros: u64,
}

#[derive(Clone)]
pub struct DefinitionSite {
  file: String,
  line: u32,
}

impl DefinitionSite {
  pub fn new(file: String, line: u32) -> DefinitionSite {
    DefinitionSite {
      file: file,
      line: line,
    }
  }
}

#[macro_export]
macro_rules! defined_here {
  () => {
    $crate::DefinitionSite::new(file!().to_owned(), line!())
  }
}


pub struct MonitoringServiceConfig {
  port: u16
}

impl Default for MonitoringServiceConfig {
  fn default() -> MonitoringServiceConfig {
    MonitoringServiceConfig {
      port: monitoring_daemon_port::CONFIG.get_value() as u16,
    }
  }
}

pub struct MonitoringService {
  config: MonitoringServiceConfig,
}

impl Default for MonitoringService {
  fn default() -> MonitoringService {
    MonitoringService {
      config: MonitoringServiceConfig::default()
    }
  }
}

struct MonitoringServlet;

impl hyper::server::Service for MonitoringServlet {
  type Request = hyper::Request;
  type Response = hyper::Response;
  type Error = hyper::Error;
  type Future = futures::future::FutureResult<hyper::Response, hyper::Error>;

  fn call(&self, req: Self::Request) -> Self::Future {
    futures::future::ok(match (req.method(), req.path()) {
      (&hyper::Get, "/_mon") => {
        hyper::Response::new()
            .with_header(hyper::header::ContentLength("test".len() as u64))
            .with_body("test")
      },
      _ => {
          Self::Response::new()
              .with_status(hyper::StatusCode::NotFound)
      }
    })
  }
}

impl MonitoringService {
  pub fn run_forever(self) {
    let addr = format!("127.0.0.1:{}", self.config.port).parse().unwrap();
    let mut server = hyper::server::Http::new().bind(&addr, || Ok(MonitoringServlet)).unwrap();
    server.no_proto();
    info!("Monitoring service running on {}", server.local_addr().unwrap());
    server.run().unwrap();
  }
}
