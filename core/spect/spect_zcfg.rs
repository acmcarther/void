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

pub struct ZcfgSpectDataRendererParams {}

pub struct ZcfgSpectDataRenderer {
  params: ZcfgSpectDataRendererParams,
  data_fetcher: ZcfgSpectDataFetcher,
  last_updated: Option<Instant>,
  data: ZcfgData,
}

pub struct ZcfgSpectDataFetcherParams {}

pub struct ZcfgSpectDataFetcher {
  params: ZcfgSpectDataFetcherParams,
}

pub struct ZcfgDetailRow {
  pub config_name: String,
  pub location: String,
  pub description: String,
  // TODO(acmcarther): Value
}

pub struct ZcfgData {
  pub detail_rows: Vec<ZcfgDetailRow>,
}

impl Default for ZcfgSpectDataRendererParams {
  fn default() -> ZcfgSpectDataRendererParams {
    ZcfgSpectDataRendererParams {}
  }
}

impl Default for ZcfgSpectDataRenderer {
  fn default() -> ZcfgSpectDataRenderer {
    ZcfgSpectDataRenderer::new(
      ZcfgSpectDataRendererParams::default(),
      ZcfgSpectDataFetcher::default(),
    )
  }
}

impl ZcfgSpectDataRenderer {
  pub fn new(
    params: ZcfgSpectDataRendererParams,
    data_fetcher: ZcfgSpectDataFetcher,
  ) -> ZcfgSpectDataRenderer {
    ZcfgSpectDataRenderer {
      params: params,
      data_fetcher: data_fetcher,
      last_updated: None,
      data: ZcfgData::empty(),
    }
  }

  pub fn update_data(&mut self) {
    self.last_updated = Some(Instant::now());
    self.data = self.data_fetcher.fetch();
  }

  pub fn render(&self) -> String {
    // TODO(acmcarther): Use a templating engine or something more robust
    format!(
      "{}\n{}\n{}\n",
      self.render_subheader(),
      self.render_content(),
      self.render_subfooter()
    )
  }

  fn render_subheader(&self) -> String {
    "<h3>ZCFG Application Data</h3>".to_owned()
  }

  fn render_content(&self) -> String {
    format!(
      "<table>\n{}\n{}\n</table>",
      self.render_table_header(),
      self.render_table_contents()
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

  fn render_table_contents(&self) -> String {
    let mut contents = String::new();

    for detail_row in self.data.detail_rows.iter() {
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

impl Default for ZcfgSpectDataFetcherParams {
  fn default() -> ZcfgSpectDataFetcherParams {
    ZcfgSpectDataFetcherParams {}
  }
}

impl Default for ZcfgSpectDataFetcher {
  fn default() -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher::new(ZcfgSpectDataFetcherParams::default())
  }
}

impl ZcfgSpectDataFetcher {
  pub fn new(params: ZcfgSpectDataFetcherParams) -> ZcfgSpectDataFetcher {
    ZcfgSpectDataFetcher { params: params }
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
