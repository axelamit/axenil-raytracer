use raytracer::scene::Scene;

fn main() {
    let mut scene = Scene::new();
    scene.test_scene();
    scene.render();
}
