use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use pong::{
    Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT
};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run (&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transforms) in (&paddles, &mut transforms).join() {
        let movement = match paddle.side {
            Side::Left => input.axis_value("left_paddle"),
            Side::Right => input.axis_value("right_paddle"),
        };
        if let Some(mv_amount) = movement {
            let scale_amount = 1.2 * mv_amount as f32;
            transforms.translation[1] = (transforms.translation[1] + scale_amount)
            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
            .max(PADDLE_HEIGHT * 0.5);
        }
        }
    }
}