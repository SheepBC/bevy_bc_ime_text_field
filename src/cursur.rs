use std::time::{Duration, Instant};

use bevy::{
    ecs::{component::Component, hierarchy::Children, query::With, system::{Query, Res}},
    text::TextSpan,
    time::{Time, Timer, TimerMode}
};

use crate::text_field::{SelectChild, TextField, TextFieldPosition};

#[derive(Component)]
pub struct TextCursor{
    pub is_see: bool,
    pub stop_sec: f32,
    pub timer: Timer,
}

impl Default for TextCursor{
    fn default() -> Self {
        Self { 
            is_see: true,
            stop_sec: 3.0,
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating)
        }
    }
}

pub(crate) fn update_text_cursor_timer(
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

pub(crate) fn update_cursor(
    q_field_inform: Query<(&TextField,&Children,&TextCursor)>,
    mut q_child_text: Query<(&mut TextSpan,&mut TextFieldPosition),With<SelectChild>>,
){
    for (field,children,cursor) in q_field_inform.iter(){
        
        for child in children.iter(){
            if let Ok((mut span,position)) = q_child_text.get_mut(*child){
                if let TextFieldPosition::Select(select) = position.clone(){
                    if field.is_focuse {
                        let time = Instant::now().duration_since(field.last_change_time).as_secs_f32();
                        if time < cursor.stop_sec{
                            if select.is_empty(){
                                **span = "|".to_string();
                            }
                            else {
                                **span = "|".to_string() + &select + &"|";
                            }
                            break;
                        }
                        if cursor.is_see{
                            if select.is_empty(){
                                **span = "|".to_string();
                            }
                            else {
                                **span = "|".to_string() + &select + &"|";
                            }
                        }
                        else{
                            **span = select.clone();
                        }
                    }
                    else {
                        if span.0 != select{
                            **span = select.clone();
                        }
                    }
                }
            }
        }
    }
}
