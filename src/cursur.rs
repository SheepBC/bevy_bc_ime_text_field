use bevy::{
    ecs::{component::Component, system::{Query, Res}},
    time::{Time, Timer},
};

#[derive(Component)]
pub struct TextCursor{
    pub is_see: bool,
    pub timer: Timer
}

pub(crate) fn update_text_cursor(
    time: Res<Time>,
    mut q_cursors:Query<&mut TextCursor>
) {
    for mut text_cursor in q_cursors.iter_mut(){
        text_cursor.timer.tick(time.delta());

        if text_cursor.timer.finished() {
            text_cursor.is_see = !text_cursor.is_see;
        }
    }
}


