pub mod data;
pub mod dxf;
pub mod geom;
pub mod ops;

#[cfg(test)]
mod tests {
    use super::data::{Main, Scene};
    use super::dxf::export_scene_to_dxf;
    use super::geom::{Circle, Line, Point2};
    use super::ops::{
        AddCircleOperator, AddLineOperator, Context, DeleteEntityOperator, MoveEntityOperator,
        Operator,
    };
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
        let op_line = AddLineOperator::new("MyLine", line);

        if op_line.poll(&context) {
            op_line.execute(&mut context);
        }

        // Add a circle using an operator
        let circle = Circle::new(Point2::new(50.0, 50.0), 25.0);
        let op_circle = AddCircleOperator::new("MyCircle", circle);

        if op_circle.poll(&context) {
            op_circle.execute(&mut context);
        }

        // Move the line
        let line_id = 1; // First entity
        let op_move = MoveEntityOperator::new(line_id, 10.0, -10.0);
        if op_move.poll(&context) {
            op_move.execute(&mut context);
        }

        // Delete the circle
        let circle_id = 2; // Second entity
        let op_delete = DeleteEntityOperator::new(circle_id);
        if op_delete.poll(&context) {
            op_delete.execute(&mut context);
        }

        // Verify entities
        assert_eq!(context.main.entities.len(), 1); // Only line remains
        let scene = context.main.scenes.get(&scene_id).unwrap();
        assert_eq!(scene.entity_ids.len(), 1);

        // Export to DXF
        let dxf_content = export_scene_to_dxf(context.main, scene);
        println!("{}", dxf_content);

        // Write to file for manual check
        fs::write("test_output.dxf", dxf_content).expect("Unable to write file");
    }
}
