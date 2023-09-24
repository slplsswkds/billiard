use bevy::prelude::*;
use crate::balls::*;

#[derive(Resource)] 
pub struct GameProgressInfo {
    balls: [(u8, Vec3, bool); 16] // (Ball_number, Translation, on_table)
}

pub fn init_game_progress_info(mut commands: Commands) {commands.insert_resource(GameProgressInfo::default())}

pub fn debug_game_progress_info(
    mut progress_info: ResMut<GameProgressInfo>,
    q_balls: Query<(&Ball, &Transform)>,
) {
    progress_info.update_game_progress_info(q_balls);
    progress_info.print_balls_info();
}

impl Default for GameProgressInfo {
    fn default() -> Self {
        let mut balls: [(_, _, _); 16] = Default::default();
        for n in 0..15 {
            balls[n] = (n as u8, Vec3::splat(0.0), false)
        }
        Self { balls: balls }
    }
}

impl GameProgressInfo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    fn update_balls_info(&mut self, q_balls: Query<(&Ball, &Transform)>) {
        for (ball, transform) in q_balls.iter() {
            self.balls[ball.number as usize] = (ball.number, transform.translation, true);
        }
    }

    pub fn update_game_progress_info(
        &mut self,
        q_balls: Query<(&Ball, &Transform)>
    ) {
        self.update_balls_info(q_balls);
    }

    pub fn print_balls_info(&self) {
        self.balls.iter().for_each(|ball| println!("ball info: {:?}", ball));
    }
}