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
use std::collections::HashSet;
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

lazy_static! {
  static ref FORBIDDEN_ADDRESS_SUBPATHS: HashSet<String> = {
    let mut set = HashSet::new();
    // Root has index bound
    set.insert("/".to_owned());
    set
  };
}

/** A Spect subpage that can be rendered and updated. */
pub trait SpectRenderableSubpage {
  fn update_data(&mut self);
  fn render(&self, query_opt: Option<&str>) -> String;
}

/** Any kind of message-bearing issue */
#[derive(Debug)]
pub struct SpectGenericIssue {
  pub message: String,
}

/** A specification for how to update the data for a subpage. */
#[derive(Debug, PartialEq, Eq)]
pub enum SpectSubpageModuleRefreshPolicy {
  OnEveryLoad,
  _NonExhaustive,
}

/** Arguments for handling of a subpage module. */
pub struct SpectSubpageModuleParams {
  pub description: Option<String>,
  pub refresh_policy: SpectSubpageModuleRefreshPolicy,
}

/** A ready-to-use subpage with configuration options. */
pub struct SpectSubpageModule {
  pub address_subpath: String,
  pub params: SpectSubpageModuleParams,
  pub renderable_subpage_boxed: Box<SpectRenderableSubpage + Send>,
}

/** A subpage processor that manages all subpages. */
pub struct SpectSubpageModuleManager {
  subpage_modules: Vec<SpectSubpageModule>,
  page_mappings: HashMap<String, usize>,
}

/** A generic description of an invalid argument. */
#[derive(Debug)]
pub struct InvalidParamDetailsEntry {
  pub path: String,
  pub message: String,
  pub details: Option<Box<InvalidParamDetails>>,
  _nonexhaustive: (),
}

/** A listing of invalid arguments and details. */
#[derive(Debug)]
pub struct InvalidParamDetails {
  pub entries: Vec<InvalidParamDetailsEntry>,
  _nonexhaustive: (),
}

/** An error on initialization of a subpage module manager. */
#[derive(Debug)]
pub struct SpectSubpageModuleManagerInitErr {
  overlapping_mappings_opt: Option<HashMap<String, Vec<usize>>>,
  invalid_params_details_entries_opt: Option<Vec<InvalidParamDetailsEntry>>,
  _nonexhaustive: (),
}

/** Arguments for configuring Spect server. */
#[derive(Clone)]
pub struct SpectServerParams {
  pub addr_ipv4: String,
  pub port: u32,
}

/** A runable spect server instance. */
pub struct SpectServer {
  params: SpectServerParams,
  subpage_module_manager_rwarc: Arc<RwLock<SpectSubpageModuleManager>>,
}

/** A request-handler spun off of a SpectServer. */
// PUBLIC_FOR_TRAIT(NewService::<SpectServer>)
pub struct SpectHandler {
  #[allow(unused)] server_params: SpectServerParams,
  subpage_module_manager_rwarc: Arc<RwLock<SpectSubpageModuleManager>>,
}

/** An error on rendering using a SpectHandler. */
#[derive(Debug)]
pub enum SpectHandlerRenderError {
  UnknownSubpath(String),
  _NonExhaustive,
}

impl Default for SpectSubpageModuleRefreshPolicy {
  fn default() -> SpectSubpageModuleRefreshPolicy {
    SpectSubpageModuleRefreshPolicy::OnEveryLoad
  }
}

impl SpectSubpageModuleRefreshPolicy {
  pub fn validate(&self) -> Option<SpectGenericIssue> {
    if self == &SpectSubpageModuleRefreshPolicy::_NonExhaustive {
      Some(SpectGenericIssue {
        message: "`_NonExhaustive` variant used".to_owned(),
      })
    } else {
      None
    }
  }
}

impl Default for SpectSubpageModuleParams {
  fn default() -> SpectSubpageModuleParams {
    SpectSubpageModuleParams {
      description: None,
      refresh_policy: SpectSubpageModuleRefreshPolicy::default(),
    }
  }
}

impl SpectSubpageModuleParams {
  pub fn validate(&self) -> Option<InvalidParamDetails> {
    let mut invalid_param_details_entries = Vec::new();
    if let Some(SpectGenericIssue { message, .. }) = self.refresh_policy.validate() {
      invalid_param_details_entries.push(InvalidParamDetailsEntry {
        path: "refresh_policy".to_owned(),
        message: message,
        details: None,
        _nonexhaustive: (),
      });
    }

    if invalid_param_details_entries.is_empty() {
      None
    } else {
      Some(InvalidParamDetails {
        entries: invalid_param_details_entries,
        _nonexhaustive: (),
      })
    }
  }
}

impl SpectSubpageModule {
  pub fn validate(&self) -> Option<InvalidParamDetails> {
    let mut invalid_param_details_entries = Vec::new();
    if FORBIDDEN_ADDRESS_SUBPATHS.contains(&self.address_subpath) {
      invalid_param_details_entries.push(InvalidParamDetailsEntry {
        path: "address_subpath".to_owned(),
        message: format!("[{}] is a banned address subpath", self.address_subpath),
        details: None,
        _nonexhaustive: (),
      });
    }

    if let Some(invalid_param_details) = self.params.validate() {
      invalid_param_details_entries.push(InvalidParamDetailsEntry {
        path: "params".to_owned(),
        message: "invalid param, see details.".to_owned(),
        details: Some(Box::new(invalid_param_details)),
        _nonexhaustive: (),
      });
    }

    if invalid_param_details_entries.is_empty() {
      None
    } else {
      Some(InvalidParamDetails {
        entries: invalid_param_details_entries,
        _nonexhaustive: (),
      })
    }
  }
}

impl SpectSubpageModuleManager {
  pub fn new(
    subpage_modules: Vec<SpectSubpageModule>,
  ) -> Result<SpectSubpageModuleManager, SpectSubpageModuleManagerInitErr> {
    let mut page_mappings = HashMap::new();

    let mut err_overlapping_mappings: HashMap<String, Vec<usize>> = HashMap::new();
    let mut err_invalid_param_details_entries = Vec::new();

    for (idx, subpage_module) in subpage_modules.iter().enumerate() {
      // Exit if params are bad
      if let Some(invalid_param_details) = subpage_module.validate() {
        err_invalid_param_details_entries.push(InvalidParamDetailsEntry {
          path: format!("subpage_modules[{}]", idx),
          message: "invalid param, see details.".to_owned(),
          details: Some(Box::new(invalid_param_details)),
          _nonexhaustive: (),
        });
        continue;
      }

      // Try insert, and exit if we didn't overwrite something
      let overlapping_idx_opt = page_mappings.insert(subpage_module.address_subpath.clone(), idx);
      if overlapping_idx_opt.is_none() {
        continue;
      }

      // If overwrite already happened before, add this instance as well
      let overlapping_idx = overlapping_idx_opt.unwrap();
      if err_overlapping_mappings.contains_key(&subpage_module.address_subpath) {
        // UNWRAP: guaranteed to be present from above guard
        let current_overlapping_idxs = err_overlapping_mappings
          .get_mut(&subpage_module.address_subpath)
          .unwrap();

        current_overlapping_idxs.push(idx);

        continue;
      }

      // If overwrite is new, add this index and prior to a new list
      let current_overlapping_idxs = vec![overlapping_idx, idx];
      err_overlapping_mappings.insert(
        subpage_module.address_subpath.clone(),
        current_overlapping_idxs,
      );
    }

    if !err_overlapping_mappings.is_empty() || !err_invalid_param_details_entries.is_empty() {
      let overlapping_mappings_opt = if !err_overlapping_mappings.is_empty() {
        Some(err_overlapping_mappings)
      } else {
        None
      };

      let invalid_params_details_entries_opt = if !err_invalid_param_details_entries.is_empty() {
        Some(err_invalid_param_details_entries)
      } else {
        None
      };

      return Err(SpectSubpageModuleManagerInitErr {
        overlapping_mappings_opt: overlapping_mappings_opt,
        invalid_params_details_entries_opt: invalid_params_details_entries_opt,
        _nonexhaustive: (),
      });
    }

    Ok(SpectSubpageModuleManager {
      page_mappings: page_mappings,
      subpage_modules: subpage_modules,
    })
  }

  pub fn modules(&self) -> &Vec<SpectSubpageModule> {
    &self.subpage_modules
  }

  pub fn has_module_for_subpath(&self, path: &String) -> bool {
    self.page_mappings.contains_key(path)
  }

  pub fn maybe_update(&mut self, path: &String) {
    if let Some(subpage_module_idx) = self.page_mappings.get(path) {
      // UNWRAP: Guaranteed to be present by construction
      let subpage_module = self.subpage_modules.get_mut(*subpage_module_idx).unwrap();
      match &subpage_module.params.refresh_policy {
        &SpectSubpageModuleRefreshPolicy::OnEveryLoad => {
          subpage_module.renderable_subpage_boxed.update_data()
        }
        // UNREACHABLE: Not constructable (verified in this struct)
        &SpectSubpageModuleRefreshPolicy::_NonExhaustive => unreachable!(),
      }
    } else {
      warn!("Tried to update for non-existent path [{}]", path);
    }
  }

  pub fn render_opt(&self, path: &String, query_opt: Option<&str>) -> Option<String> {
    self.page_mappings.get(path).map(|subpage_module_idx| {
      let subpage_module = self.subpage_modules.get(*subpage_module_idx).unwrap();
      subpage_module.renderable_subpage_boxed.render(query_opt)
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
      self.subpage_module_manager_rwarc.clone(),
    ))
  }
}

impl SpectServer {
  pub fn new(
    params: SpectServerParams,
    subpage_module_manager: SpectSubpageModuleManager,
  ) -> SpectServer {
    SpectServer {
      params: params,
      subpage_module_manager_rwarc: Arc::new(RwLock::new(subpage_module_manager)),
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
    subpage_module_manager_rwarc: Arc<RwLock<SpectSubpageModuleManager>>,
  ) -> SpectHandler {
    SpectHandler {
      server_params: server_params,
      subpage_module_manager_rwarc: subpage_module_manager_rwarc,
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
      self.render_header(uri.path()),
      rendered_subpage,
      self.render_footer()
    );

    Ok(rendered_page)
  }

  fn render_content_for(&self, uri: &Uri) -> Option<String> {
    let path = uri.path().to_owned();

    // Special case for root -- Render nothing (but let header render)
    if path == "/" {
      return Some("".to_owned());
    }

    // UNWRAP: Not handling poisoned RWLock
    let has_module = self
      .subpage_module_manager_rwarc
      .read()
      .unwrap()
      .has_module_for_subpath(&path);

    if !has_module {
      return None;
    }

    // UNWRAP: Not handling poisoned RWLock
    let mut subpage_module_manager = self.subpage_module_manager_rwarc.write().unwrap();
    subpage_module_manager.maybe_update(&path);

    // UNWRAP: known to exist by `has_module` check above
    // N.B: This could just yield the possibly wrapped value, but that makes the semantics less
    // clear
    let rendered_subpage = subpage_module_manager
      .render_opt(&path, uri.query())
      .unwrap();

    Some(rendered_subpage)
  }

  fn render_header(&self, path: &str) -> String {
    format!(
      "<html>\n{}\n{}\n<body>",
      self.render_title(path),
      self.render_index(),
    )
  }

  fn render_title(&self, path: &str) -> String {
    format!("<h1>Spect @ \"{}\"</h1>", path)
  }

  fn render_index(&self) -> String {
    // UNWRAP: Not handling poisoned RWLock
    let subpage_module_manager = self.subpage_module_manager_rwarc.read().unwrap();
    let modules = subpage_module_manager.modules();

    let mut rendered_index_items = Vec::new();

    // Special case for root
    rendered_index_items.push(format!("<li><a href=\"/\">[root]</a>: Index</li>"));

    for module in modules {
      rendered_index_items.push(format!(
        "<li><a href=\"{}\">{}</a>: {}</li>",
        module.address_subpath,
        module.address_subpath,
        module
          .params
          .description
          .as_ref()
          .map(|s| s.clone())
          .unwrap_or_else(|| "No Description".to_owned())
      ));
    }

    format!(
      "<h3>Spect Index</h3><ul>\n{}\n</ul>",
      rendered_index_items.join("\n")
    )
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
