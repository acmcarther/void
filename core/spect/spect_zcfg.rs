#![feature(used)]

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate spect;
#[macro_use]
extern crate zcfg;

use spect::SpectRenderableSubpage;
use spect::SpectSubpageModule;
use spect::SpectSubpageModuleParams;
use std::time::Instant;

define_pub_cfg!(
  spect_zcfg_config_name_hide_prefix,
  String,
  "__".to_owned(),
  "Configs named with this leading prefix will be hidden from listing."
);

pub struct ZcfgSpectRenderableSubpage {
  data_fetcher: ZcfgSpectDataFetcher,
  data_renderer: ZcfgSpectDataRenderer,
  last_updated: Option<Instant>,
  data: ZcfgData,
}

pub struct ZcfgSpectDataRenderer {}

pub struct ZcfgSpectDataFetcher {
  config_name_hide_prefix: String,
}

pub struct ZcfgDetailRow {
  pub config_name: String,
  pub location: String,
  pub description: String,
  // TODO(acmcarther): Value
}

pub struct ZcfgData {
  pub detail_rows: Vec<ZcfgDetailRow>,
  pub hidden_detail_rows: Vec<ZcfgDetailRow>,
}

impl Default for ZcfgSpectRenderableSubpage {
  fn default() -> ZcfgSpectRenderableSubpage {
    ZcfgSpectRenderableSubpage::new(
      ZcfgSpectDataFetcher::default(),
      ZcfgSpectDataRenderer::default(),
    )
  }
}

impl SpectRenderableSubpage for ZcfgSpectRenderableSubpage {
  fn update_data(&mut self) {
    self.last_updated = Some(Instant::now());
    self.data = self.data_fetcher.fetch();
  }

  fn render(&self, _query_opt: Option<&str>) -> String {
    self.data_renderer.render(&self.data)
  }
}

impl ZcfgSpectRenderableSubpage {
  pub fn new(
    data_fetcher: ZcfgSpectDataFetcher,
    data_renderer: ZcfgSpectDataRenderer,
  ) -> ZcfgSpectRenderableSubpage {
    ZcfgSpectRenderableSubpage {
      data_fetcher: data_fetcher,
      data_renderer: data_renderer,
      last_updated: None,
      data: ZcfgData::empty(),
    }
  }
}

impl Default for ZcfgSpectDataRenderer {
  fn default() -> ZcfgSpectDataRenderer {
    ZcfgSpectDataRenderer::new()
  }
}

impl ZcfgSpectDataRenderer {
  pub fn new() -> ZcfgSpectDataRenderer {
    ZcfgSpectDataRenderer {}
  }

  pub fn render(&self, data: &ZcfgData) -> String {
    // TODO(acmcarther): Use a templating engine or something more robust
    format!(
      "{}\n{}\n{}\n",
      self.render_subheader(),
      self.render_content(&data),
      self.render_subfooter(&data)
    )
  }

  fn render_subheader(&self) -> String {
    "<h3>ZCFG Application Data</h3>".to_owned()
  }

  fn render_content(&self, data: &ZcfgData) -> String {
    format!(
      "<table>\n{}\n{}\n</table>",
      self.render_table_header(),
      self.render_table_contents(&data)
    )
  }

  fn render_table_header(&self) -> String {
    r#"
    <tr>
      <th>Name</th>
      <th>Location</th>
      <th>Description</th>
    </tr>"#.to_owned()
  }

  fn render_table_contents(&self, data: &ZcfgData) -> String {
    let mut contents = String::new();
    for detail_row in data.detail_rows.iter() {
      contents.push_str("    <tr>\n");
      contents.push_str("      <td>");
      contents.push_str(&detail_row.config_name);
      contents.push_str("</td>\n");
      contents.push_str("      <td>");
      contents.push_str(&detail_row.location);
      contents.push_str("</td>\n");
      contents.push_str("      <td>");
      contents.push_str(&detail_row.description);
      contents.push_str("</td>\n");
      contents.push_str("    </tr>\n");
    }

    contents
  }

  fn render_subfooter(&self, data: &ZcfgData) -> String {
    format!("<p>And {} hidden</p>", data.hidden_detail_rows.len())
  }
}

impl Default for ZcfgSpectDataFetcher {
  fn default() -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher::new(spect_zcfg_config_name_hide_prefix::CONFIG.get_value())
  }
}

impl ZcfgSpectDataFetcher {
  pub fn new(config_name_hide_prefix: String) -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher {
      config_name_hide_prefix: config_name_hide_prefix,
    }
  }

  pub fn fetch(&mut self) -> ZcfgData {
    // UNWRAP: If this fails, the application is blown.
    let initializers = zcfg::STATIC_CONFIG_INITIALIZERS.read().unwrap();

    let mut detail_rows = Vec::new();
    let mut hidden_detail_rows = Vec::new();

    for initr in initializers.iter() {
      let row = ZcfgDetailRow {
        config_name: initr.config_name().to_owned(),
        location: format!("{}:{}", initr.file().to_owned(), initr.line()),
        description: initr.description().to_owned(),
      };
      if !row.config_name.starts_with(&self.config_name_hide_prefix) {
        detail_rows.push(row);
      } else {
        hidden_detail_rows.push(row);
      }
    }

    ZcfgData {
      detail_rows: detail_rows,
      hidden_detail_rows: hidden_detail_rows,
    }
  }
}

impl ZcfgData {
  pub fn empty() -> ZcfgData {
    ZcfgData {
      detail_rows: Vec::new(),
      hidden_detail_rows: Vec::new(),
    }
  }
}

pub fn get_zcfg_subpage_module() -> SpectSubpageModule {
  SpectSubpageModule {
    address_subpath: "/zcfg".to_owned(),
    params: SpectSubpageModuleParams {
      description: Some("Zcfg flag configuration details".to_owned()),
      ..SpectSubpageModuleParams::default()
    },
    renderable_subpage_boxed: Box::new(ZcfgSpectRenderableSubpage::default()),
  }
}
