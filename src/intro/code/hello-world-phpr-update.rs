// ANCHOR: entry-point
// ANCHOR: class-declaration
use godot::prelude::*;
// ANCHOR_END: class-declaration

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
// ANCHOR_END: entry-point

// ANCHOR: class-declaration
use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f32,
    angular_speed: f32,

    base: Base<Sprite2D>,
}
// ANCHOR_END: class-declaration

// ANCHOR: init
// ANCHOR: physics-process
// ANCHOR: rotate
use godot::classes::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    // ANCHOR_END: init
    // ANCHOR_END: physics-process
    // ANCHOR_END: rotate
    // ANCHOR: init
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f32::consts::PI,
            base,
        }
    }
    // ANCHOR_END: init

    // This snippet contains the updated physics_process block.
    // delta may also be f64.
    // ANCHOR: physics-process
    fn physics_process(&mut self, delta: f32) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        let radians = self.angular_speed * delta;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);

        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
    // ANCHOR_END: physics-process
    // ANCHOR: init
}
// ANCHOR_END: init
// ANCHOR_END: rotate

// amount is allowed to be f64 too
// ANCHOR: custom-api
#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f32) {
        self.speed += amount;
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
// ANCHOR_END: custom-api
