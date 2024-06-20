use crate::entities::SelectOptionCellDataPB;
use crate::services::field::SelectOptionIds;
use collab::core::any_map::{AnyMap, AnyMapExtension};
use collab_database::database::gen_option_id;
use serde::{Deserialize, Serialize};

/// [SelectOption] represents an option for a single select, and multiple select.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
  pub id: String,
  pub name: String,
  pub color: SelectOptionColor,
}

impl SelectOption {
  pub fn new(name: &str) -> Self {
    SelectOption {
      id: gen_option_id(),
      name: name.to_owned(),
      color: SelectOptionColor::default(),
    }
  }

  pub fn with_color(name: &str, color: SelectOptionColor) -> Self {
    SelectOption {
      id: gen_option_id(),
      name: name.to_owned(),
      color,
    }
  }
}

impl From<AnyMap> for SelectOption {
  fn from(map: AnyMap) -> Self {
    SelectOption {
      id: map.get_str_value("id").unwrap_or_default(),
      name: map.get_str_value("name").unwrap_or_default(),
      color: SelectOptionColor::from(map.get_i64_value("color").unwrap_or(0)),
    }
  }
}

impl From<SelectOption> for AnyMap {
  fn from(option: SelectOption) -> Self {
    let mut map = AnyMap::new();
    map.insert_str_value("id", option.id);
    map.insert_str_value("name", option.name);
    map.insert_i64_value("color", option.color.into());
    map
  }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
#[derive(Default)]
pub enum SelectOptionColor {
  #[default]
  Purple = 0,
  Pink = 1,
  LightPink = 2,
  Orange = 3,
  Yellow = 4,
  Lime = 5,
  Green = 6,
  Aqua = 7,
  Blue = 8,
}

impl From<i64> for SelectOptionColor {
  fn from(value: i64) -> Self {
    match value {
      0 => SelectOptionColor::Purple,
      1 => SelectOptionColor::Pink,
      2 => SelectOptionColor::LightPink,
      3 => SelectOptionColor::Orange,
      4 => SelectOptionColor::Yellow,
      5 => SelectOptionColor::Lime,
      6 => SelectOptionColor::Green,
      7 => SelectOptionColor::Aqua,
      8 => SelectOptionColor::Blue,
      _ => SelectOptionColor::default(),
    }
  }
}

impl From<SelectOptionColor> for i64 {
  fn from(color: SelectOptionColor) -> Self {
    color as i64
  }
}

#[derive(Debug)]
pub struct SelectOptionCellData {
  pub select_options: Vec<SelectOption>,
}

impl From<SelectOptionCellData> for SelectOptionCellDataPB {
  fn from(data: SelectOptionCellData) -> Self {
    SelectOptionCellDataPB {
      select_options: data
        .select_options
        .into_iter()
        .map(|option| option.into())
        .collect(),
    }
  }
}

pub fn make_selected_options(ids: SelectOptionIds, options: &[SelectOption]) -> Vec<SelectOption> {
  ids
    .iter()
    .flat_map(|option_id| {
      options
        .iter()
        .find(|option| &option.id == option_id)
        .cloned()
    })
    .collect()
}
