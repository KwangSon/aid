pub mod data;
pub mod dxf;
pub mod geom;
pub mod ops;

#[cfg(test)]
mod tests {
    use super::data::{Main, Scene};
    use super::dxf::export_scene_to_dxf;
    use super::geom::{Line, Point2};
    use super::ops::{AddLineOperator, Context, Operator};
    use std::fs;

    #[test]
    fn test_v0_flow() {
        let mut main = Main::new();

        // Create a scene
        let scene_id = main.generate_id();
        let scene = Scene::new(scene_id, "MainScene");
        main.add_scene(scene);

        // Setup context
        let mut context = Context {
            main: &mut main,
            active_scene_id: scene_id,
        };

        // Add a line using an operator
        let line = Line::new(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0));
        let op = AddLineOperator::new("MyLine", line);

        if op.poll(&context) {
            op.execute(&mut context);
        }

        // Verify entity was added to main and scene
        assert_eq!(context.main.entities.len(), 1);
        let scene = context.main.scenes.get(&scene_id).unwrap();
        assert_eq!(scene.entity_ids.len(), 1);

        // Export to DXF
        let dxf_content = export_scene_to_dxf(context.main, scene);
        println!("{}", dxf_content);

        // Write to file for manual check
        fs::write("test_output.dxf", dxf_content).expect("Unable to write file");
    }
}
