use tracing::{error, instrument};

use crate::pack::api::tracker::nested::item::{
    self, CompositeToggle, Consumable, Item, Progressive, ProgressiveToggle, Static, Toggle,
    ToggleBadged,
};

#[derive(Debug)]
pub struct StatefulItem {
    common: item::Common,
    variant: StatefulItemVariant,
}

impl StatefulItem {
    pub fn new(item: Item) -> Self {
        let Item { common, variant } = item;

        Self {
            common,
            variant: StatefulItemVariant::new(variant),
        }
    }

    #[instrument(level = "error")]
    pub fn provider_count(&self, item_code: &str) -> i32 {
        let common_codes_match = self.common.codes.contains(item_code);

        match &self.variant {
            StatefulItemVariant::Static { item: _ } => common_codes_match as i32,
            StatefulItemVariant::Progressive {
                item,
                active_stage_index,
                disabled,
            } => {
                if item.allow_disabled && *disabled {
                    return 0;
                }

                let Some(stages_to_check) = item.stages.get(*active_stage_index..) else {
                    error!("active stage index out of bounds");
                    return 0;
                };

                for stage in stages_to_check {
                    let stage_matches = stage.codes.contains(item_code);

                    if stage_matches {
                        return 1;
                    }

                    if !stage.inherit_codes {
                        break;
                    }
                }

                0
            }
            StatefulItemVariant::Toggle { item: _, enabled } => {
                if !common_codes_match {
                    return 0;
                }

                if *enabled {
                    1
                } else {
                    0
                }
            }
            StatefulItemVariant::Consumable { item: _, count } => {
                if !common_codes_match {
                    return 0;
                }

                *count
            }
            StatefulItemVariant::ProgressiveToggle {
                item,
                active_stage_index,
            } => {
                let Some(stages_to_check) = item.stages.get(*active_stage_index..) else {
                    error!("active stage index out of bounds");
                    return 0;
                };

                for stage in stages_to_check {
                    let stage_matches = stage.codes.contains(item_code);

                    if stage_matches {
                        return 1;
                    }

                    if !stage.inherit_codes {
                        break;
                    }
                }

                0
            }
            StatefulItemVariant::CompositeToggle { item, left, right } => {
                let mut left_count = 0;
                let mut right_count = 0;

                if *left && item.item_left == item_code {
                    left_count = 1;
                }

                if *right && item.item_right == item_code {
                    right_count = 1;
                }

                for image in &item.images {
                    if *left && image.codes.contains(item_code) {
                        left_count = 1;
                    }

                    if *right && image.codes.contains(item_code) {
                        right_count = 1;
                    }
                }

                left_count + right_count
            }
            StatefulItemVariant::ToggleBadged { item: _, enabled } => {
                if !common_codes_match {
                    return 0;
                }

                if *enabled {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum StatefulItemVariant {
    Static {
        item: Static,
    },
    Progressive {
        item: Progressive,
        active_stage_index: usize,
        disabled: bool,
    },
    Toggle {
        item: Toggle,
        enabled: bool,
    },
    Consumable {
        item: Consumable,
        count: i32,
    },
    ProgressiveToggle {
        item: ProgressiveToggle,
        active_stage_index: usize,
    },
    CompositeToggle {
        item: CompositeToggle,
        left: bool,
        right: bool,
    },
    ToggleBadged {
        item: ToggleBadged,
        enabled: bool,
    },
}

impl StatefulItemVariant {
    fn new(variant: item::Variant) -> Self {
        match variant {
            item::Variant::Static(item) => Self::Static { item },
            item::Variant::Progressive(item) => Self::Progressive {
                active_stage_index: item.initial_stage_idx,
                disabled: item.allow_disabled,
                item,
            },
            item::Variant::Toggle(item) => Self::Toggle {
                enabled: item.initial_active_state,
                item,
            },
            item::Variant::Consumable(item) => Self::Consumable {
                count: item.initial_quantity,
                item,
            },
            item::Variant::ProgressiveToggle(item) => Self::ProgressiveToggle {
                active_stage_index: item.initial_stage_idx,
                item,
            },
            item::Variant::CompositeToggle(item) => Self::CompositeToggle {
                item,
                left: false,
                right: false,
            },
            item::Variant::ToggleBadged(item) => Self::ToggleBadged {
                enabled: item.initial_active_state,
                item,
            },
        }
    }
}
