use crate::world::loot_box::Loot;

pub fn ldtk_loot_enum_to_rust_enum(ldtk_enum_field: &String) -> Loot {
    if ldtk_enum_field == "GrapplingHook" {
        Loot::GrapplingHook
    } else {
        panic!("Unknown loot enum from ldtk: {}", ldtk_enum_field);
    }
}
