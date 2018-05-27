#![feature(used)]

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate zcfg;

use futures::future::Future;
use hyper::Result as HyperResult;
use hyper::Uri;
use hyper::header::ContentLength;
use hyper::server::Http;
use hyper::server::NewService;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Service;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::RwLock;

define_pub_cfg!(
  spect_server_addr_ipv4,
  String,
  "127.0.0.1",
  "Host addr ipv4 to run from."
);

define_pub_cfg!(
  spect_server_port,
  u32,
  3663u32,
  "Port to run the spect server on"
);

const PHRASE: &'static str = "Hello, World!";

#[derive(Debug)]
pub struct SpectGenericIssue {
  message: String,
}

pub trait SpectRenderableSubpage {
  fn update_data(&mut self);
  fn render(&self, query_opt: Option<&str>) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpectPageModuleRefreshPolicy {
  OnEveryLoad,
  _NonExhaustive,
}

pub struct SpectPageModuleParams {
  pub refresh_policy: SpectPageModuleRefreshPolicy,
}

pub struct SpectPageModule {
  pub address_subpath: String,
  pub params: SpectPageModuleParams,
  pub renderable_subpage_boxed: Box<SpectRenderableSubpage>,
}

pub struct SpectPageModuleManager {
  page_modules: Vec<SpectPageModule>,
  page_mappings: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct InvalidParamDetailsEntry {
  pub path: String,
  pub message: String,
  pub details: Option<Box<InvalidParamDetails>>,
  _nonexhaustive: (),
}

#[derive(Debug)]
pub struct InvalidParamDetails {
  pub entries: Vec<InvalidParamDetailsEntry>,
  _nonexhaustive: (),
}

#[derive(Debug)]
pub struct SpectPageModuleManagerInitErr {
  overlapping_mappings_opt: Option<HashMap<String, Vec<usize>>>,
  invalid_params_details_list_opt: Option<Vec<InvalidParamDetails>>,
  _nonexhaustive: (),
}

#[derive(Clone)]
pub struct SpectServerParams {
  pub addr_ipv4: String,
  pub port: u32,
}

pub struct SpectServer {
  params: SpectServerParams,
  page_module_manager_rwarc: Arc<RwLock<SpectPageModuleManager>>,
}

// PUBLIC_FOR_TRAIT(NewService::<SpectServer>)
pub struct SpectHandler {
  server_params: SpectServerParams,
  page_module_manager_rwarc: Arc<RwLock<SpectPageModuleManager>>,
}

#[derive(Debug)]
pub enum SpectHandlerRenderError {
  UnknownSubpath(String),
  _NonExhaustive,
}

impl Default for SpectPageModuleRefreshPolicy {
  fn default() -> SpectPageModuleRefreshPolicy {
    SpectPageModuleRefreshPolicy::OnEveryLoad
  }
}

impl SpectPageModuleRefreshPolicy {
  pub fn validate(&self) -> Option<SpectGenericIssue> {
    if self == &SpectPageModuleRefreshPolicy::_NonExhaustive {
      Some(SpectGenericIssue {
        message: "`_NonExhaustive` variant used".to_owned(),
      })
    } else {
      None
    }
  }
}

impl Default for SpectPageModuleParams {
  fn default() -> SpectPageModuleParams {
    SpectPageModuleParams {
      refresh_policy: SpectPageModuleRefreshPolicy::default(),
    }
  }
}

impl SpectPageModuleParams {
  pub fn validate(&self) -> Option<InvalidParamDetails> {
    if let Some(SpectGenericIssue { message, .. }) = self.refresh_policy.validate() {
      Some(InvalidParamDetails {
        entries: vec![
          InvalidParamDetailsEntry {
            path: "refresh_policy".to_owned(),
            message: message,
            details: None,
            _nonexhaustive: (),
          },
        ],
        _nonexhaustive: (),
      })
    } else {
      None
    }
  }
}

impl SpectPageModuleManager {
  pub fn new(
    page_modules: Vec<SpectPageModule>,
  ) -> Result<SpectPageModuleManager, SpectPageModuleManagerInitErr> {
    let mut page_mappings = HashMap::new();

    let mut err_overlapping_mappings: HashMap<String, Vec<usize>> = HashMap::new();
    let mut err_invalid_params_details_list = Vec::new();

    for (idx, page_module) in page_modules.iter().enumerate() {
      // Exit if params are bad
      if let Some(invalid_param_details) = page_module.params.validate() {
        err_invalid_params_details_list.push(invalid_param_details);
        continue;
      }

      // Try insert, and exit if we didn't overwrite something
      let overlapping_idx_opt = page_mappings.insert(page_module.address_subpath.clone(), idx);
      if overlapping_idx_opt.is_none() {
        continue;
      }

      // If overwrite already happened before, add this instance as well
      let overlapping_idx = overlapping_idx_opt.unwrap();
      if err_overlapping_mappings.contains_key(&page_module.address_subpath) {
        // UNWRAP: guaranteed to be present from above guard
        let current_overlapping_idxs = err_overlapping_mappings
          .get_mut(&page_module.address_subpath)
          .unwrap();

        current_overlapping_idxs.push(idx);

        continue;
      }

      // If overwrite is new, add this index and prior to a new list
      let current_overlapping_idxs = vec![overlapping_idx, idx];
      err_overlapping_mappings.insert(
        page_module.address_subpath.clone(),
        current_overlapping_idxs,
      );
    }

    if !err_overlapping_mappings.is_empty() || !err_invalid_params_details_list.is_empty() {
      let overlapping_mappings_opt = if !err_overlapping_mappings.is_empty() {
        Some(err_overlapping_mappings)
      } else {
        None
      };

      let invalid_params_details_list_opt = if !err_invalid_params_details_list.is_empty() {
        Some(err_invalid_params_details_list)
      } else {
        None
      };

      return Err(SpectPageModuleManagerInitErr {
        overlapping_mappings_opt: overlapping_mappings_opt,
        invalid_params_details_list_opt: invalid_params_details_list_opt,
        _nonexhaustive: (),
      });
    }

    Ok(SpectPageModuleManager {
      page_mappings: page_mappings,
      page_modules: page_modules,
    })
  }

  pub fn has_module_for_subpath(&self, path: &String) -> bool {
    self.page_mappings.contains_key(path)
  }

  pub fn maybe_update(&mut self, path: &String) {
    if let Some(page_module_idx) = self.page_mappings.get(path) {
      // UNWRAP: Guaranteed to be present by construction
      let page_module = self.page_modules.get_mut(*page_module_idx).unwrap();
      match &page_module.params.refresh_policy {
        &SpectPageModuleRefreshPolicy::OnEveryLoad => {
          page_module.renderable_subpage_boxed.update_data()
        }
        // UNREACHABLE: Not constructable (verified in this struct)
        &SpectPageModuleRefreshPolicy::_NonExhaustive => unreachable!(),
      }
    } else {
      warn!("Tried to update for non-existent path [{}]", path);
    }
  }

  pub fn render_opt(&self, path: &String, query_opt: Option<&str>) -> Option<String> {
    self.page_mappings.get(path).map(|page_module_idx| {
      let page_module = self.page_modules.get(*page_module_idx).unwrap();
      page_module.renderable_subpage_boxed.render(query_opt)
    })
  }
}

impl Default for SpectServerParams {
  fn default() -> SpectServerParams {
    SpectServerParams {
      addr_ipv4: spect_server_addr_ipv4::CONFIG.get_value(),
      port: spect_server_port::CONFIG.get_value(),
    }
  }
}

impl NewService for SpectServer {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Instance = SpectHandler;

  fn new_service(&self) -> io::Result<SpectHandler> {
    Ok(SpectHandler::new(
      self.params.clone(),
      self.page_module_manager_rwarc.clone(),
    ))
  }
}

impl SpectServer {
  pub fn new(
    params: SpectServerParams,
    page_module_manager: SpectPageModuleManager,
  ) -> SpectServer {
    SpectServer {
      params: params,
      page_module_manager_rwarc: Arc::new(RwLock::new(page_module_manager)),
    }
  }

  pub fn run(self) -> HyperResult<()> {
    let addr = format!("{}:{}", self.params.addr_ipv4, self.params.port)
      .parse()
      .unwrap();
    info!("Running SpectServer on [{}]", addr);
    let server = Http::new().bind(&addr, self).unwrap();
    server.run()
  }
}

impl SpectHandler {
  pub fn new(
    server_params: SpectServerParams,
    page_module_manager_rwarc: Arc<RwLock<SpectPageModuleManager>>,
  ) -> SpectHandler {
    SpectHandler {
      server_params: server_params,
      page_module_manager_rwarc: page_module_manager_rwarc,
    }
  }

  pub fn try_render_page(&self, uri: &Uri) -> Result<String, SpectHandlerRenderError> {
    let module_content_opt = self.render_content_for(&uri);

    if module_content_opt.is_none() {
      return Err(SpectHandlerRenderError::UnknownSubpath(
        uri.path().to_owned(),
      ));
    }
    // UNWRAP: Guarded above
    let rendered_subpage = module_content_opt.unwrap();
    let rendered_page = format!(
      "{}\n{}\n{}",
      self.render_header(),
      rendered_subpage,
      self.render_footer()
    );

    Ok(rendered_page)
  }

  fn render_content_for(&self, uri: &Uri) -> Option<String> {
    let path = uri.path().to_owned();
    // UNWRAP: Not handling poisoned RWLock
    let has_module = self
      .page_module_manager_rwarc
      .read()
      .unwrap()
      .has_module_for_subpath(&path);

    if !has_module {
      return None;
    }

    // UNWRAP: Not handling poisoned RWLock
    let mut page_module_manager = self.page_module_manager_rwarc.write().unwrap();
    page_module_manager.maybe_update(&path);

    // UNWRAP: known to exist by `has_module` check above
    // N.B: This could just yield the possibly wrapped value, but that makes the semantics less
    // clear
    let rendered_subpage = page_module_manager.render_opt(&path, uri.query()).unwrap();

    Some(rendered_subpage)
  }

  fn render_header(&self) -> String {
    "<html><body>".to_owned()
  }

  fn render_footer(&self) -> String {
    "</body></html>".to_owned()
  }
}

impl Service for SpectHandler {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;

  type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let rendered_page_res = self.try_render_page(req.uri());
    match rendered_page_res {
      Ok(rendered_page) => Box::new(futures::future::ok(
        Response::new()
          .with_header(ContentLength(rendered_page.len() as u64))
          .with_body(rendered_page),
      )),
      Err(err) => {
        let err_str = format!("{:?}", err);
        Box::new(futures::future::ok(
          Response::new()
            .with_header(ContentLength(err_str.len() as u64))
            .with_body(err_str),
        ))
      }
    }
  }
}
