#![feature(used)]

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate spect;
#[macro_use]
extern crate zcfg;

use hyper::Result as HyperResult;
use spect::SpectRenderableSubpage;
use std::time::Instant;

pub struct ZcfgSpectRenderableSubpage {
  data_fetcher: ZcfgSpectDataFetcher,
  data_renderer: ZcfgSpectDataRenderer,
  last_updated: Option<Instant>,
  data: ZcfgData,
}

pub struct ZcfgSpectDataRenderer {}

pub struct ZcfgSpectDataFetcher {}

pub struct ZcfgDetailRow {
  pub config_name: String,
  pub location: String,
  pub description: String,
  // TODO(acmcarther): Value
}

pub struct ZcfgData {
  pub detail_rows: Vec<ZcfgDetailRow>,
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
  fn try_update_data(&mut self, _force_update: bool) {
    // TODO(acmcarther): Use internal params to configure max update frequency
    self.last_updated = Some(Instant::now());
    self.data = self.data_fetcher.fetch();
  }

  fn render(&self) -> String {
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
      self.render_subfooter()
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

  fn render_subfooter(&self) -> String {
    "".to_owned()
  }
}

impl Default for ZcfgSpectDataFetcher {
  fn default() -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher::new()
  }
}

impl ZcfgSpectDataFetcher {
  pub fn new() -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher {}
  }

  pub fn fetch(&mut self) -> ZcfgData {
    // UNWRAP: If this fails, the application is blown.
    let initializers = zcfg::STATIC_CONFIG_INITIALIZERS.read().unwrap();

    let detail_rows = initializers
      .iter()
      .map(|initr| ZcfgDetailRow {
        config_name: initr.config_name().to_owned(),
        location: format!("{}:{}", initr.file().to_owned(), initr.line()),
        description: initr.description().to_owned(),
      })
      .collect::<Vec<_>>();
    ZcfgData {
      detail_rows: detail_rows,
    }
  }
}

impl ZcfgData {
  pub fn empty() -> ZcfgData {
    ZcfgData {
      detail_rows: Vec::new(),
    }
  }
}
