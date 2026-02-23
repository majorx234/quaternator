use nalgebra_glm as glm;
use std::f32::consts::PI;

fn main() {
    // 1. Create a 3D vector (x, y, z)
    let position = glm::vec3(1.0, 0.0, 0.0);

    // 2. Create a translation matrix (move by 2 units along x)
    let translation = glm::translation(&glm::vec3(2.0, 0.0, 0.0));

    // 3. Create a rotation matrix (90 degrees around z-axis)
    let rotation = glm::rotation(PI / 2.0, &glm::vec3(0.0, 0.0, 1.0));

    // 4. Combine transformations: Rotate then Translate
    let model_matrix = translation * rotation;

    // 5. Apply transformation to the vector (using homogeneous coordinates)
    // We append a 1.0 to make it a point (x, y, z, 1.0)
    let point = glm::vec4(position.x, position.y, position.z, 1.0);
    let transformed_point = model_matrix * point;

    println!("Original Position: {:?}", position);
    println!("Transformed Point: {:?}", transformed_point); // Should be (2, 1, 0, 1)

    // 6. Perspective Projection Matrix
    let projection = glm::perspective(
        4.0 / 3.0, // Aspect Ratio
        PI / 4.0,  // Field of View (45 degrees)
        0.1,       // Near plane
        100.0      // Far plane
    );
    println!("Projection Matrix:\n{}", projection);
}
