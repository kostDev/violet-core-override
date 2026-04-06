mod engine;
mod types;


#[repr(C)]
pub struct LootStoreItem {
    pub item_id: u32,   // id of the item
    pub reference: i32, // referenced TemplateleId
    pub chance: f32, // chance to drop for both quest and non-quest items, chance to be used for refs
    pub needs_quest: bool, // : 1 | quest drop (quest is required for item to drop)
    pub group_id: u8,   // :7
    pub loot_mode: u16,
    pub min_count: u8, // min count for drop items
    pub max_count: u8, // max drop count for the item mincount or Ref multiplicator
    // ConditionList conditions;                   // additional loot condition
}

// #[repr(C)]
// pub struct LootItem {
//     pub item_id: u32,
//     pub item_index: u32,
//     pub random_suffix: u32,
//     pub random_property_id: i32,
//     //     ConditionList conditions;                               // additional loot condition
//     //     AllowedLooterSet allowedGUIDs;
//     //     ObjectGuid rollWinnerGUID;                              // Stores the guid of person who won loot, if his bags are full only he can see the item in loot list!
//     pub chance: f32,
//     pub min_count: i32,
//     pub max_count: i32,
//  //     uint8   count             : 8;
//     //     bool    is_looted         : 1;
//     //     bool    is_blocked        : 1;
//     //     bool    freeforall        : 1;                          // free for all
//     //     bool    is_underthreshold : 1;
//     //     bool    is_counted        : 1;
//     //     bool    needs_quest       : 1;                          // quest drop
//     //     bool    follow_loot_rules : 1;
//     //     uint8   groupid           : 7;
// }