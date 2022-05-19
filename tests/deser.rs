use ldtk_easy::{entity::Field, project::Project};

#[test]
fn test_deser() {
    let json = String::from_utf8(include_bytes!("../assets/example.ldtk").to_vec()).unwrap();
    let project = Project::new(json).unwrap();

    let level = project.get_level("Level_0").unwrap();
    let layer = level.get_layer("Entities").unwrap();
    let entities = layer.entities();

    let harvest_hero = entities
        .iter()
        .find(|e| e.identifier() == String::from("HarvestHero"));

    assert_eq!(true, harvest_hero.is_some());

    let harvest_hero = harvest_hero.unwrap();

    let fields = harvest_hero.fields();

    let float_array = fields.get("Float");
    assert_eq!(true, float_array.is_some());
    let float_array = float_array.unwrap();
    match float_array {
        Field::Array { value } => {
            assert_eq!(0, value.len());
        }
        _ => panic!("Expected a float array"),
    }

    let color = fields.get("Color");
    assert_eq!(true, color.is_some());
    let color = color.unwrap();
    match color {
        Field::String { value } => {
            assert_eq!("#000000", value);
        }
        _ => panic!("Expected a float array"),
    }
}
