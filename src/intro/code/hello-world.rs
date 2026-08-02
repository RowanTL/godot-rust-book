// Copyright (c) godot-rust; Bromeon and contributors.
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Complete source for the Hello World tutorial. Every code block in
// src/intro/hello-world.md includes one of the anchors below.
//
// Anchor names may be opened and closed more than once; mdbook concatenates the regions and
// hides everything in between. That is how a block can show e.g. the `impl` header and one
// method without the other methods.

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
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
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
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
    // ANCHOR_END: init

    // ANCHOR: physics-process
    // ANCHOR: rotate
    fn physics_process(&mut self, delta: f64) {
        // ANCHOR_END: physics-process
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        // ANCHOR: physics-process
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        // ANCHOR_END: rotate

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);

        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    // ANCHOR: rotate
    }
    // ANCHOR_END: physics-process
// ANCHOR: init
// ANCHOR: physics-process
}
// ANCHOR_END: init
// ANCHOR_END: physics-process
// ANCHOR_END: rotate

// ANCHOR: custom-api
#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
// ANCHOR_END: custom-api
