use std::path::PathBuf;
use std::collections::HashMap;
use log;
use cpp_data::{CppData, CppTypeKind, CppVisibility};
use caption_strategy::MethodCaptionStrategy;
use cpp_method::{CppMethod, CppMethodKind};
use cpp_type::CppTypeBase;
use cpp_code_generator::CppCodeGenerator;
use cpp_ffi_data::CppAndFfiMethod;

pub struct CGenerator {
  lib_path: PathBuf,
  lib_name: String,
  cpp_data: CppData,
  template_classes: Vec<String>,
  abstract_classes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CppFfiHeaderData {
  pub include_file: String,
  pub include_file_base_name: String,
  pub methods: Vec<CppAndFfiMethod>,
}

pub struct CppAndFfiData {
  pub cpp_data: CppData,
  pub cpp_data_by_headers: HashMap<String, CppData>,
  pub cpp_ffi_headers: Vec<CppFfiHeaderData>,
}

impl CGenerator {
  pub fn new(cpp_data: CppData, lib_name: String, lib_path: PathBuf) -> Self {
    CGenerator {
      lib_path: lib_path,
      lib_name: lib_name,
      template_classes: cpp_data.types
        .clone()
        .iter()
        .filter_map(|t| {
          if cpp_data.is_template_class(&t.name) {
            Some(t.name.clone())
          } else {
            None
          }
        })
        .collect(),
      cpp_data: cpp_data,
      abstract_classes: Vec::new(),
    }
  }

  pub fn generate_all(mut self) -> CppAndFfiData {
    let code_gen = CppCodeGenerator::new(self.lib_name.clone(), self.lib_path.clone());
    self.abstract_classes = self.cpp_data
      .types
      .iter()
      .filter_map(|t| {
        if let CppTypeKind::Class { .. } = t.kind {
          if self.get_pure_virtual_methods(&t.name).len() > 0 {
            Some(t.name.clone())
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();
    log::info(format!("Abstract classes: {:?}", self.abstract_classes));

    let mut c_headers = Vec::new();
    let cpp_data_by_headers = self.cpp_data.split_by_headers();
    let mut include_name_list = Vec::new();

    for (ref include_file, ref data) in &cpp_data_by_headers {
      if include_file == &"QFlags" || include_file == &"QFlag" {
        log::info(format!("Skipping include file {}", include_file));
        continue;
      }
      let mut include_file_base_name = (*include_file).clone();
      if include_file_base_name.ends_with(".h") {
        include_file_base_name = include_file_base_name[0..include_file_base_name.len() - 2]
          .to_string();
      }
      let methods = self.process_methods(&include_file, &include_file_base_name, &data.methods);
      c_headers.push(CppFfiHeaderData {
        include_file: (*include_file).clone(),
        include_file_base_name: include_file_base_name,
        methods: methods,
      });
      include_name_list.push((*include_file).clone());
    }
    c_headers.sort_by(|a, b| a.include_file.cmp(&b.include_file));
    code_gen.generate_all_headers_file(&include_name_list);
    for data in &c_headers {
      code_gen.generate_one(data);
    }

    CppAndFfiData {
      cpp_data: self.cpp_data,
      cpp_data_by_headers: cpp_data_by_headers,
      cpp_ffi_headers: c_headers,
    }

  }


  #[allow(dead_code)]
  fn get_all_methods(&self, class_name: &String) -> Vec<CppMethod> {
    let own_methods: Vec<_> = self.cpp_data
      .methods
      .iter()
      .filter(|m| m.class_name() == Some(class_name))
      .collect();
    let mut inherited_methods = Vec::new();
    if let Some(type_info) = self.cpp_data.types.iter().find(|t| &t.name == class_name) {
      if let CppTypeKind::Class { ref bases, .. } = type_info.kind {
        for base in bases {
          if let CppTypeBase::Class { ref name, .. } = base.base {
            for method in self.get_all_methods(name) {
              if own_methods.iter()
                .find(|m| m.name == method.name && m.argument_types_equal(&method))
                .is_none() {
                inherited_methods.push(method.clone());
              }
            }
          }
        }
      } else {
        panic!("get_all_methods: not a class");
      }
    } else {
      log::warning(format!("get_all_methods: no type info for {:?}", class_name));
    }
    for m in own_methods {
      inherited_methods.push((*m).clone());
    }
    inherited_methods
  }

  fn get_pure_virtual_methods(&self, class_name: &String) -> Vec<CppMethod> {

    let own_methods: Vec<_> = self.cpp_data
      .methods
      .iter()
      .filter(|m| m.class_name() == Some(class_name))
      .collect();
    let own_pure_virtual_methods: Vec<_> = own_methods.iter()
      .filter(|m| {
        m.class_membership
          .as_ref()
          .unwrap()
          .is_pure_virtual
      })
      .collect();
    let mut inherited_methods = Vec::new();
    if let Some(type_info) = self.cpp_data.types.iter().find(|t| &t.name == class_name) {
      if let CppTypeKind::Class { ref bases, .. } = type_info.kind {
        for base in bases {
          if let CppTypeBase::Class { ref name, .. } = base.base {
            for method in self.get_pure_virtual_methods(name) {
              if own_methods.iter()
                .find(|m| m.name == method.name && m.argument_types_equal(&method))
                .is_none() {
                inherited_methods.push(method.clone());
              }
            }
          }
        }
      } else {
        panic!("get_pure_virtual_methods: not a class");
      }
    } else {
      log::warning(format!("get_pure_virtual_methods: no type info for {:?}",
                           class_name));
    }
    for m in own_pure_virtual_methods {
      inherited_methods.push((*m).clone());
    }
    inherited_methods
  }

  pub fn process_methods(&self,
                         include_file: &String,
                         include_file_base_name: &String,
                         methods: &Vec<CppMethod>)
                         -> Vec<CppAndFfiMethod> {
    log::info(format!("Generating C++ FFI methods for header: <{}>", include_file));
    let mut hash1 = HashMap::new();
    {
      let insert_into_hash = |hash: &mut HashMap<String, Vec<_>>, key: String, value| {
        if let Some(values) = hash.get_mut(&key) {
          values.push(value);
          return;
        }
        hash.insert(key, vec![value]);
      };

      for ref method in methods {
        if let Some(ref membership) = method.class_membership {
          if membership.kind == CppMethodKind::Constructor {
            let class_name = membership.class_type.maybe_name().unwrap();
            if self.abstract_classes.iter().find(|x| x == &class_name).is_some() {
              log::debug(format!("Method is skipped:\n{}\nConstructors are not allowed for \
                                  abstract classes.\n",
                                 method.short_text()));
              continue;
            }
          }
          if membership.visibility == CppVisibility::Private {
            continue;
          }
          if membership.visibility == CppVisibility::Protected {
            log::debug(format!("Skipping protected method: \n{}\n", method.short_text()));
            continue;
          }
          if membership.is_signal {
            log::warning(format!("Skipping signal: \n{}\n", method.short_text()));
            continue;
          }
        }
        if method.template_arguments.is_some() {
          log::warning(format!("Skipping template method: \n{}\n", method.short_text()));
          continue;
        }
        if let Some(ref class_name) = method.class_name() {
          if self.template_classes
            .iter()
            .find(|x| x == class_name || class_name.starts_with(&format!("{}::", x)))
            .is_some() {
            log::warning(format!("Skipping method of template class: \n{}\n",
                                 method.short_text()));
            continue;
          }
        }

        match method.to_ffi_signatures() {
          Err(msg) => {
            log::warning(format!("Unable to produce C function for method:\n{}\nError:{}\n",
                                 method.short_text(),
                                 msg));
          }
          Ok(results) => {
            for result in results {
              match result.c_base_name(include_file_base_name) {
                Err(msg) => {
                  log::warning(format!("Unable to produce C function for method:\n{}\nError:{}\n",
                                       method.short_text(),
                                       msg));
                }
                Ok(name) => {
                  insert_into_hash(&mut hash1, name, result);
                }
              }
            }
          }
        }
      }
    }
    let mut r = Vec::new();
    for (key, mut values) in hash1.into_iter() {
      if values.len() == 1 {
        r.push(CppAndFfiMethod::new(values.remove(0), key.clone()));
        continue;
      }
      let mut found_strategy = None;
      for strategy in MethodCaptionStrategy::all() {
        let mut type_captions: Vec<_> = values.iter()
          .map(|x| x.caption(strategy.clone()))
          .collect();
        type_captions.sort();
        type_captions.dedup();
        if type_captions.len() == values.len() {
          found_strategy = Some(strategy);
          break;
        }
      }
      if let Some(strategy) = found_strategy {
        for x in values {
          let caption = x.caption(strategy.clone());
          r.push(CppAndFfiMethod::new(x,
                                      format!("{}{}{}",
                                              key,
                                              if caption.is_empty() { "" } else { "_" },
                                              caption)));
        }
      } else {
        panic!("all type caption strategies have failed! Involved functions: \n{:?}",
               values);
      }
    }
    r.sort_by(|a, b| a.c_name.cmp(&b.c_name));
    r
  }
}
