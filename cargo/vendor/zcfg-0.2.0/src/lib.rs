#![feature(used)]

#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
use std::ops::Deref;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::Formatter;

#[macro_export]
macro_rules! define_cfg {
  ($name:ident, $cfg_type:ty, $default_value:expr, $description:expr) => {
    #[allow(dead_code)]
    mod $name {
      _define_config_inner!($name, $cfg_type, $default_value, $description);
    }
  };
}

#[macro_export]
macro_rules! define_pub_cfg {
  ($name:ident, $cfg_type:ty, $default_value:expr, $description:expr) => {
    #[allow(dead_code)]
    pub mod $name {
      _define_config_inner!($name, $cfg_type, $default_value, $description);
    }
  };
}

#[macro_export]
macro_rules! _define_config_inner {
  ($name:ident, $cfg_type:ty, $default_value:expr, $description:expr) => {
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::ops::Deref;
    use std::convert::Into;

    lazy_static! {
      /** The accessible static value for this modules flag. */
      pub static ref CONFIG: $crate::Config<$cfg_type> = {
        $crate::Config::__new_cfg_in_macro_do_not_use_elsewhere(
          stringify!($name).to_owned(),
          file!().to_owned(),
          line!(),
          $description,
          $default_value.into(),
          __inner_get,
          __inner_set_for_testing
        )
      };

      /**
       * An additional initializer to allow config users to force intialization of a config.
       *
       * For general use prefer an aggregate initializer that populates all configs in the binary
       * at once.
       * For testing, prefer `CONFIG.set_for_testing`.
       */
      pub static ref INITIALIZER: $crate::ConfigInitializer = {
        $crate::ConfigInitializer::__new_init_in_macro_do_not_use_elsewhere(
          stringify!($name).to_owned(),
          file!().to_owned(),
          line!(),
          $description,
          __inner_try_set_statically,
        )
      };

      static ref _CONFIG_INNER: Arc<RwLock<$crate::__ConfigValue<$cfg_type>>> = {
        Arc::new(RwLock::new($crate::__ConfigValue::new($default_value.into())))
      };
    }

    fn __inner_try_set_statically(s: &str) -> Result<(), $crate::InitErr> {
      <$cfg_type as $crate::ConfigParseable>::parse_from_str(s)
        .map_err(|e| $crate::InitErr::FailedToParse(e.to_string()))
        .and_then(|out| {
          if !_CONFIG_INNER.write().expect("somebody soiled a config").initialize(out) {
            Err($crate::InitErr::AlreadyInitOnce)
          } else {
            Ok(())
          }
        })
    }

    fn __inner_get() -> $cfg_type {
      _CONFIG_INNER.deref().read().expect("somebody soiled a config").get()
    }

    fn __inner_set_for_testing(v: $cfg_type) {
      _CONFIG_INNER.deref().write().expect("somebody soiled a config").set_raw(v)
    }


    extern "C" fn enqueue_static_config_init() {
      $crate::STATIC_CONFIG_INITIALIZERS.write()
        .unwrap()
        .push($crate::ConfigInitializer::__new_init_in_macro_do_not_use_elsewhere(
          stringify!($name).to_owned(),
          file!().to_owned(),
          line!(),
          $description,
          __inner_try_set_statically,
        ))
    }

    #[used]
    #[cfg_attr(target_os = "linux", link_section = ".init_array")]
    #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
    #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
    #[allow(dead_code)]
    static INIT_ARRAY: [extern "C" fn(); 1] = [enqueue_static_config_init];
  }
}

define_pub_cfg!(__zcfg_example_pub_config, super::NoneableCfg<String>, None,
            "A fake pub example configuration object to demo rustdoc for config objects. The \
            naming convention is completely artificial, and you can choose any convention you'd \
            like. Prefer distinct names that probably won't conflict with other libraries.");
define_cfg!(__zcfg_example_priv_config, super::NoneableCfg<String>, None,
            "A fake example configuration object to demo rustdoc for config objects. The \
            naming convention is completely artificial, and you can choose any convention you'd \
            like. Prefer distinct names that probably won't conflict with other libraries.");

/** A configurable element in the associated module */
pub struct Config<T: Clone> {
  name: String,
  file_name: String,
  line_number: u32,
  description: &'static str,
  default_value: T,
  _inner_get_value: fn() -> T,
  _inner_set_for_testing: fn(T),
}

impl<T: Clone> Config<T> {
  /**
   * Constructs a new config object for the known static initializer retrieval methods.
   *
   * This is public by necessity for access by the `declare_cfg` macros.
   */
  pub fn __new_cfg_in_macro_do_not_use_elsewhere(
      name: String,
      file_name: String,
      line_number: u32,
      description: &'static str,
      default_value: T,
      get_value: fn() -> T,
      set_for_testing: fn(T)) 
      -> Config<T> {
    Config {
      name: name,
      file_name: file_name,
      line_number: line_number,
      description: description,
      default_value: default_value,
      _inner_get_value: get_value,
      _inner_set_for_testing: set_for_testing,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn file_name(&self) -> &str {
    &self.file_name
  }

  pub fn line_number(&self) -> u32 {
    self.line_number
  }

  pub fn description(&self) -> &str {
    self.description
  }

  /**
   * Fetch the current value for this config from global state.
   *
   * It may or may not be initialized.
   *
   * To increase testability you should limit retrieval of config values to some
   * dependency resolution area of your code, rather than within business logic.
   */
  pub fn get_value(&self) -> T {
    (self._inner_get_value)()
  }

  /**
   * Sets the value of this config directly for testing.
   *
   * Concurrent tests that invoke this method will probably be flakey. Prefer to set this in a
   * std::sync::ONCE block, and verify that it has been called.
   */
  pub fn set_for_testing(&self, v: T) {
    (self._inner_set_for_testing)(v)
  }

  /**
   * Resets the value of this config back to its default directly for testing.
   *
   * Concurrent tests that invoke this method will probably be flakey. Prefer to set this in a
   * std::sync::ONCE block, and verify that it has been called.
   */
  pub fn reset_for_testing(&self) {
    (self._inner_set_for_testing)(self.default_value.clone())
  }
}

pub type ParseErr = String;

/** Any standard type which may be parsed from a string */
pub trait ConfigParseable {
  type Output;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr>;
}

macro_rules! decl_config_parsable_from_str {
  ($auto_parsable_type:ty) => {
    impl ConfigParseable for $auto_parsable_type {
      type Output = $auto_parsable_type;
      fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
        use std::str::FromStr;
        <$auto_parsable_type as FromStr>::from_str(s).map_err(|e| e.to_string())
      }
    }
  }
}

decl_config_parsable_from_str!(String);
decl_config_parsable_from_str!(bool);
decl_config_parsable_from_str!(u8);
decl_config_parsable_from_str!(u32);
decl_config_parsable_from_str!(u64);
decl_config_parsable_from_str!(i8);
decl_config_parsable_from_str!(i32);
decl_config_parsable_from_str!(i64);
decl_config_parsable_from_str!(f32);
decl_config_parsable_from_str!(f64);



#[derive(Clone)]
#[deprecated(since="0.2.0", note="please use Option directly")]
pub struct NoneableCfg<T>(pub Option<T>);

impl <T> NoneableCfg<T> {
  pub fn inner(self) -> Option<T> {
    let NoneableCfg(inner) = self;
    inner
  }
}

impl <T> Debug for NoneableCfg<T> where T: Debug {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    let &NoneableCfg(ref inner) = self;
    inner.fmt(f)
  }
}

impl <T> Deref for NoneableCfg<T> {
  type Target = Option<T>;

  fn deref(&self) -> &Option<T> {
    let &NoneableCfg(ref inner) = self;
    inner
  }
}

impl <T> From<Option<T>> for NoneableCfg<T> {
  fn from(v: Option<T>) -> Self {
    NoneableCfg(v)
  }
}

impl <T> ConfigParseable for NoneableCfg<T> where T:ConfigParseable {
  type Output = NoneableCfg<<T as ConfigParseable>::Output>;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
    let opt_res = match s {
      // TODO(acmcarther): Be more comprehensive here
      "None" | "none" => {
        Ok(None)
      }
      s => {
        <T as ConfigParseable>::parse_from_str(s).map(|v| Some(v))
      }
    };
    opt_res.map(|v| NoneableCfg(v))
  }
}

impl <T> ConfigParseable for Option<T> where T:ConfigParseable {
  type Output = Option<<T as ConfigParseable>::Output>;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
    match s {
      // TODO(acmcarther): Be more comprehensive here
      "None" | "none" => {
        Ok(None)
      }
      s => <T as ConfigParseable>::parse_from_str(s).map(|v| Some(v))
    }
  }
}


#[derive(Clone)]
#[deprecated(since="0.2.0", note="please use Vec directly")]
pub struct CommaSeparatedCfgs<T>(pub Vec<T>);

impl <T> CommaSeparatedCfgs<T> {
  pub fn inner(self) -> Vec<T> {
    let CommaSeparatedCfgs(inner) = self;
    inner
  }
}

impl <T> Deref for CommaSeparatedCfgs<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Vec<T> {
    let &CommaSeparatedCfgs(ref inner) = self;
    inner
  }
}

impl <T> From<Vec<T>> for CommaSeparatedCfgs<T> {
  fn from(v: Vec<T>) -> Self {
    CommaSeparatedCfgs(v)
  }
}

impl <T> Debug for CommaSeparatedCfgs<T> where T: Debug {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    let &CommaSeparatedCfgs(ref inner) = self;
    inner.fmt(f)
  }
}

impl <T> ConfigParseable for CommaSeparatedCfgs<T> where T:ConfigParseable {
  type Output = CommaSeparatedCfgs<<T as ConfigParseable>::Output>;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
    let mut results = Vec::new();
    for element in s.split(',') {
      let parsed = <T as ConfigParseable>::parse_from_str(element);
      if parsed.is_err() {
        return Err(parsed.err().unwrap())
      } else {
        results.push(parsed.ok().unwrap())
      }
    }
    return Ok(CommaSeparatedCfgs(results))
  }
}

impl <T> ConfigParseable for Vec<T> where T:ConfigParseable {
  type Output = Vec<<T as ConfigParseable>::Output>;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
    let mut results = Vec::new();
    for element in s.split(',') {
      let parsed = <T as ConfigParseable>::parse_from_str(element);
      if parsed.is_err() {
        return Err(parsed.err().unwrap())
      } else {
        results.push(parsed.ok().unwrap())
      }
    }
    return Ok(results)
  }
}


/**
 * The inner config value.
 *
 * This value is public to allow access by `define_cfg` macros.
 */
pub struct __ConfigValue<T: Clone> {
  value: T,
  initialized: bool
}

impl<T: Clone> __ConfigValue<T> {
  pub fn new(default: T) -> __ConfigValue<T> {
    __ConfigValue {
      value: default,
      initialized: false
    }
  }
  pub fn get(&self) -> T {
    self.value.clone()
  }

  pub fn set_raw(&mut self, t: T) {
    self.value = t;
  }

  pub fn initialize(&mut self, t: T) -> bool {
    if self.initialized {
      return false
    }

    self.set_raw(t);
    self.initialized = true;
    true
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InitErr {
  AlreadyInitOnce,
  FailedToParse(String),
}

/**
 * An initialization object that can statically set config objects.
 *
 * Opaquely enqueuing these objects into a global queue allows config resolution crates to
 * parse out configuration without knowing the entire set of transitive dependencies.
 *
 * You will find an initializer available in the declared config module, named "INITIALIZER".
 * For the purpose of testing, prefer to access the config value directly (named "CONFIG"), and
 * invoke `.set_for_testing` directly.
 * For general application initialization, prefer a global config intializer invoked in main.
 */
pub struct ConfigInitializer {
  metadata: ConfigMetadata,
  internal_set_statically: fn(&str) -> Result<(), InitErr>,
}

impl ConfigInitializer {
  /**
   * Constructs a new initializer from known initializer reference.
   *
   * This function is public to allow access by `define_cfg` macros.
   */
  pub fn __new_init_in_macro_do_not_use_elsewhere(
      name: String,
      file_name: String,
      line_number: u32,
      description: &'static str,
      initialize: fn(&str) -> Result<(), InitErr>)
      -> ConfigInitializer {
    ConfigInitializer {
      metadata: ConfigMetadata::new(name, file_name, line_number, description),
      internal_set_statically: initialize,
    }
  }

  pub fn config_name(&self) -> &str {
    self.metadata.config_name()
  }

  pub fn file(&self) -> &str {
    self.metadata.file()
  }

  pub fn line(&self) -> u32 {
    self.metadata.line()
  }

  pub fn description(&self) -> &str {
    self.metadata.description()
  }

  pub fn metadata(&self) -> &ConfigMetadata {
    &self.metadata
  }

  /**
   * Attempts to parse the config from a string and load it into the associated Config value.
   *
   * Global initializers use this function after loading global configuration from some source,
   * typically command line arguments or a config file, to populate Config values.
   */
  pub fn set_statically(&self, s: &str) -> Result<(), InitErr> {
    (self.internal_set_statically)(s)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigMetadata {
  config_name: String,
  file_name: String,
  line_number: u32,
  description: &'static str,
}

impl ConfigMetadata {
  pub fn new(
      config_name: String,
      file_name: String,
      line_number: u32,
      description: &'static str)
      -> ConfigMetadata {
    ConfigMetadata {
      config_name: config_name,
      file_name: file_name,
      line_number: line_number,
      description: description
    }
  }

  pub fn config_name(&self) -> &str {
    &self.config_name
  }

  pub fn file(&self) -> &str {
    &self.file_name
  }

  pub fn line(&self) -> u32 {
    self.line_number
  }

  pub fn description(&self) -> &str {
    self.description
  }
}

lazy_static! {
  /**
   * The exhaustive list of config initializer objects.
   *
   * This list is populated by the constitutients of this binary before the main function (using
   * very scary linker flags).
   */
  pub static ref STATIC_CONFIG_INITIALIZERS: RwLock<Vec<ConfigInitializer>> = {
    RwLock::new(Vec::new())
  };
}

#[cfg(test)]
mod test {
  pub use super::*;
  define_cfg!(example_1, String, "hello".to_owned(), "some example configuration");
  define_cfg!(example_2, u32, 5u32, "some example_2 configuration");
  define_pub_cfg!(example_3, super::NoneableCfg<String>, None, "some example_3 configuration");
  define_pub_cfg!(example_4, super::NoneableCfg<u32>, None, "some example_4 configuration");
  define_pub_cfg!(example_5, Option<String>, None, "some example_5 configuration");
  define_pub_cfg!(example_6, Vec<u32>, Vec::new(), "some example_6 configuration");
  use self::example_1::CONFIG as CONFIG_example_1;
  use self::example_2::CONFIG as CONFIG_example_2;
  use self::example_3::CONFIG as CONFIG_example_3;
  use self::example_4::CONFIG as CONFIG_example_4;
  use self::example_5::CONFIG as CONFIG_example_5;
  use self::example_6::CONFIG as CONFIG_example_6;
  use std::sync::Mutex;

  lazy_static! {
    static ref NO_TEST_PARALLELISM: Mutex<()>= { Mutex::new(()) };
  }

  fn reset_world() {
    CONFIG_example_1.reset_for_testing();
    CONFIG_example_2.reset_for_testing();
    CONFIG_example_3.reset_for_testing();
    CONFIG_example_4.reset_for_testing();
  }

  #[test]
  fn set_for_testing_works() {
    #[allow(unused_variables)]
    let l = NO_TEST_PARALLELISM.lock();
    reset_world();

    CONFIG_example_1.set_for_testing("goodbye".to_owned());
    assert_eq!(CONFIG_example_1.get_value(), "goodbye".to_owned());
  }

  #[test]
  fn reset_for_testing_works() {
    #[allow(unused_variables)]
    let l = NO_TEST_PARALLELISM.lock();
    // A little suspicious -- we're relying on resetting to work for the reset test
    reset_world();

    CONFIG_example_1.set_for_testing("goodbye".to_owned());
    assert_eq!(CONFIG_example_1.get_value(), "goodbye".to_owned());
    CONFIG_example_1.reset_for_testing();
    assert_eq!(CONFIG_example_1.get_value(), "hello".to_owned());
  }

  #[test]
  fn unset_flags_are_unset() {
    #[allow(unused_variables)]
    let l = NO_TEST_PARALLELISM.lock();
    reset_world();

    assert_eq!(CONFIG_example_3.get_value().inner(), None);
    assert_eq!(CONFIG_example_4.get_value().inner(), None);
  }

  #[test]
  fn basic_collections_work() {
    #[allow(unused_variables)]
    let l = NO_TEST_PARALLELISM.lock();
    reset_world();

    assert_eq!(CONFIG_example_5.get_value(), None);
    assert_eq!(CONFIG_example_6.get_value(), Vec::new());
  }

  #[test]
  fn global_initializer_contains_all_flags() {
    #[allow(unused_variables)]
    let l = NO_TEST_PARALLELISM.lock();
    reset_world();

    let mut static_config_names = STATIC_CONFIG_INITIALIZERS.read()
      .expect("some other test ruined the initializer")
      .iter()
      .map(|initializer| initializer.config_name().to_owned())
      .collect::<Vec<String>>();

    static_config_names.sort();

    let expected_values = vec![
      "__zcfg_example_priv_config".to_owned(),
      "__zcfg_example_pub_config".to_owned(),
      "example_1".to_owned(),
      "example_2".to_owned(),
      "example_3".to_owned(),
      "example_4".to_owned(),
      "example_5".to_owned(),
      "example_6".to_owned(),
    ];

    assert_eq!(static_config_names, expected_values);
  }
}
