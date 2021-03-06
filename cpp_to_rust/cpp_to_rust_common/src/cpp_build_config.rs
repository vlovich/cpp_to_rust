//! Types for configuring build script behavior.

use errors::Result;

/// Information required to build the C++ wrapper library
/// on every supported platform. it contains list of linked
/// libraries, frameworks, compiler types and selected type of
/// C++ wrapper library (shared or static). Default value of this
/// object is set before generation of the crate using
/// `cpp_to_rust_generator::config::Config::set_cpp_build_config` or
/// `cpp_build_config_mut` and intended to be cross-platform.
///
/// In order to allow target-dependent build configuration,
/// multiple configurations can be added to one `CppBuildConfig` object,
/// each with a condition.
/// During evaluation, each configuration item
/// will only be used if the associated condition is currently true.
/// All properties from all matching configuration are combined.
///
/// If this conditional evaluation is not enough, a custom build script
/// can modify this config during build script execution using
/// `cpp_to_rust_build_tools::Config::set_cpp_build_config` or
/// `cpp_build_config_mut`.
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CppBuildConfig {
  items: Vec<CppBuildConfigItem>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
struct CppBuildConfigItem {
  condition: ::target::Condition,
  data: CppBuildConfigData,
}

/// Type of a C++ library (shared or static).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum CppLibraryType {
  Shared,
  Static,
}

/// Platform-specific information
/// required to build the C++ wrapper library.
/// This type contains one configuration item of `CppBuildConfig`.
#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct CppBuildConfigData {
  linked_libs: Vec<String>,
  linked_frameworks: Vec<String>,
  compiler_flags: Vec<String>,
  library_type: Option<CppLibraryType>,
}

impl CppBuildConfigData {
  /// Constructs an empty object.
  pub fn new() -> CppBuildConfigData {
    CppBuildConfigData::default()
  }

  /// Adds a library for linking. Used as `-l` option to the linker.
  pub fn add_linked_lib<P: Into<String>>(&mut self, lib: P) {
    self.linked_libs.push(lib.into());
  }

  /// Adds a framework for linking (OS X specific). Used as `-f` option to the linker.
  pub fn add_linked_framework<P: Into<String>>(&mut self, lib: P) {
    self.linked_frameworks.push(lib.into());
  }

  /// Adds a command line argument for the C++ compiler.
  pub fn add_compiler_flag<P: Into<String>>(&mut self, lib: P) {
    self.compiler_flags.push(lib.into());
  }

  /// Adds multiple flags. See `CppBuildConfigData::add_cpp_compiler_flag`.
  pub fn add_compiler_flags<Item, Iter>(&mut self, items: Iter)
    where Item: Into<String>,
          Iter: IntoIterator<Item = Item>
  {
    for item in items {
      self.compiler_flags.push(item.into());
    }
  }

  /// Sets library type. C++ wrapper is static by default.
  /// Shared library can be used to work around MSVC linker's limitations.
  pub fn set_library_type(&mut self, t: CppLibraryType) {
    self.library_type = Some(t);
  }

  /// Returns names of linked libraries.
  pub fn linked_libs(&self) -> &[String] {
    &self.linked_libs
  }

  /// Returns names of linked frameworks.
  pub fn linked_frameworks(&self) -> &[String] {
    &self.linked_frameworks
  }

  /// Returns C++ compiler flags.
  pub fn compiler_flags(&self) -> &[String] {
    &self.compiler_flags
  }

  /// Returns type of C++ wrapper libary (shared or static).
  pub fn library_type(&self) -> Option<CppLibraryType> {
    self.library_type
  }

  fn add_from(&mut self, other: &CppBuildConfigData) -> Result<()> {
    self.linked_libs.append(&mut other.linked_libs.clone());
    self
      .linked_frameworks
      .append(&mut other.linked_frameworks.clone());
    self
      .compiler_flags
      .append(&mut other.compiler_flags.clone());
    if self.library_type.is_some() {
      if other.library_type.is_some() && other.library_type != self.library_type {
        return Err("conflicting library types specified".into());
      }
    } else {
      self.library_type = other.library_type;
    }
    Ok(())
  }
}

impl CppBuildConfig {
  /// Create an empty configuration
  pub fn new() -> CppBuildConfig {
    CppBuildConfig::default()
  }
  /// Add `data` with `condition`.
  pub fn add(&mut self, condition: ::target::Condition, data: CppBuildConfigData) {
    self
      .items
      .push(CppBuildConfigItem {
              condition: condition,
              data: data,
            });
  }
  /// Select all conditions that are true on `target`, combine all corresponding
  /// configuration items and return the result.
  pub fn eval(&self, target: &::target::Target) -> Result<CppBuildConfigData> {
    let mut data = CppBuildConfigData::default();
    for item in &self.items {
      if item.condition.eval(target) {
        data.add_from(&item.data)?;
      }
    }
    Ok(data)
  }
}

use std::path::PathBuf;

/// Machine-specific information required to build the C++ wrapper library.
/// This type holds configuration properties that cannot be determined
/// at the time of crate generation because they are always platform-dependent.
///
/// By default, all path lists are empty, and the build script doesn't add
/// any extra directories to paths while compiling and linking the crate.
/// If `CPP_TO_RUST_LIB_PATHS`, `CPP_TO_RUST_FRAMEWORK_PATHS` or
/// `CPP_TO_RUST_INCLUDE_PATHS` environment variables are present during
/// execution of the build script, their values are used. A custom
/// build script can get an object of this type using `Config::cpp_build_paths_mut`
/// and use its API to set extra search paths.
///
/// This type is currently only used in `cpp_to_rust_build_tools`, but
/// `cpp_to_rust_generator` may start to use it in the future if needed.
#[derive(Debug, Default, Clone)]
pub struct CppBuildPaths {
  lib_paths: Vec<PathBuf>,
  framework_paths: Vec<PathBuf>,
  include_paths: Vec<PathBuf>,
}

impl CppBuildPaths {
  /// Constructs an empty configuration object.
  pub fn new() -> CppBuildPaths {
    CppBuildPaths::default()
  }

  /// Adds `path` to a lib directory.
  /// It's supplied to the linker via `-L` option or environment variables.
  pub fn add_lib_path<P: Into<PathBuf>>(&mut self, path: P) {
    let path = path.into();
    if !self.lib_paths.contains(&path) {
      self.lib_paths.push(path);
    }
  }

  /// Adds `path` to a framework directory (OS X specific).
  /// It's supplied to the linker via `-F` option or environment variables.
  pub fn add_framework_path<P: Into<PathBuf>>(&mut self, path: P) {
    let path = path.into();
    if !self.framework_paths.contains(&path) {
      self.framework_paths.push(path);
    }
  }

  /// Adds `path` to an include directory.
  /// It's supplied to the C++ parser
  /// and the C++ compiler via `-I` option.
  pub fn add_include_path<P: Into<PathBuf>>(&mut self, path: P) {
    let path = path.into();
    if !self.include_paths.contains(&path) {
      self.include_paths.push(path);
    }
  }

  /// If `CPP_TO_RUST_LIB_PATHS`, `CPP_TO_RUST_FRAMEWORK_PATHS` or
  /// `CPP_TO_RUST_INCLUDE_PATHS` environment variables are present,
  /// their values override current values of the object.
  pub fn apply_env(&mut self) {
    use std::env;
    if let Ok(paths) = env::var("CPP_TO_RUST_LIB_PATHS") {
      self.lib_paths = env::split_paths(&paths).collect();
    }
    if let Ok(paths) = env::var("CPP_TO_RUST_FRAMEWORK_PATHS") {
      self.framework_paths = env::split_paths(&paths).collect();
    }
    if let Ok(paths) = env::var("CPP_TO_RUST_INCLUDE_PATHS") {
      self.include_paths = env::split_paths(&paths).collect();
    }
  }

  /// Returns paths added via `add_lib_path`.
  pub fn lib_paths(&self) -> &[PathBuf] {
    &self.lib_paths
  }

  /// Returns paths added via `add_framework_path`.
  pub fn framework_paths(&self) -> &[PathBuf] {
    &self.framework_paths
  }

  /// Returns paths added via `add_include_path`.
  pub fn include_paths(&self) -> &[PathBuf] {
    &self.include_paths
  }
}
