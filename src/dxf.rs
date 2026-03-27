use crate::data::{EntityData, Main, Scene};
use std::fmt::Write;

pub fn export_scene_to_dxf(main: &Main, scene: &Scene) -> String {
    let mut dxf = String::new();

    // HEADER
    writeln!(dxf, "  0\nSECTION\n  2\nHEADER\n  0\nENDSEC").unwrap();

    // ENTITIES
    writeln!(dxf, "  0\nSECTION\n  2\nENTITIES").unwrap();

    for &entity_id in &scene.entity_ids {
        if let Some(entity) = main.get_entity(entity_id) {
            match &entity.data {
                EntityData::Line(line) => {
                    writeln!(dxf, "  0\nLINE").unwrap();
                    writeln!(dxf, "  8\n0").unwrap(); // Layer 0
                    writeln!(dxf, " 10\n{}", line.start.x).unwrap();
                    writeln!(dxf, " 20\n{}", line.start.y).unwrap();
                    writeln!(dxf, " 11\n{}", line.end.x).unwrap();
                    writeln!(dxf, " 21\n{}", line.end.y).unwrap();
                }
                EntityData::Circle(circle) => {
                    writeln!(dxf, "  0\nCIRCLE").unwrap();
                    writeln!(dxf, "  8\n0").unwrap(); // Layer 0
                    writeln!(dxf, " 10\n{}", circle.center.x).unwrap();
                    writeln!(dxf, " 20\n{}", circle.center.y).unwrap();
                    writeln!(dxf, " 40\n{}", circle.radius).unwrap();
                }
            }
        }
    }

    writeln!(dxf, "  0\nENDSEC").unwrap();

    // EOF
    writeln!(dxf, "  0\nEOF").unwrap();

    dxf
}
