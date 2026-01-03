
use std::time::{Duration, Instant};

use bevy::{
    ecs::{
        component::Component,
        hierarchy::Children,
        query::With,
        system::{Query, Res},
    },
    text::TextSpan,
    time::{Time, Timer, TimerMode},
};
use crate::text_field::{SelectChild, TextFieldInfo, TextFieldInput, TextFieldPosition};
use crate::text_field_style::{change_passwd, TextFieldStyle};

#[derive(Component)]
pub struct TextFieldSelection {
    pub display: bool,
    pub stop_time: f32,
    pub change_timer: Timer,
}

impl Default for TextFieldSelection {
    fn default() -> Self {
        Self {
            display: true,
            stop_time: 3.0,
            change_timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
        }
    }
}

pub(crate) fn update_text_cursor_timer(
    time: Res<Time>,
    mut q_cursors: Query<&mut TextFieldSelection>,
) {
    for mut text_cursor in q_cursors.iter_mut() {
        text_cursor.change_timer.tick(time.delta());

        if text_cursor.change_timer.is_finished() {
            text_cursor.display = !text_cursor.display;
        }
    }
}

pub(crate) fn update_cursor(
    q_field_inform: Query<(
        &TextFieldInfo,
        &TextFieldInput,
        &TextFieldStyle,
        &Children,
        &TextFieldSelection,
    )>,
    mut q_child_text: Query<(&mut TextSpan, &mut TextFieldPosition), With<SelectChild>>,
) {
    let now = Instant::now();
    for (field_info, input, style,children, cursor) in q_field_inform.iter() {
        for child in children.iter() {
            
            let Ok((mut span, position)) = q_child_text.get_mut(*child) else { continue };
            let TextFieldPosition::Select(select) = position.clone() else { continue };
            
            if field_info.focus {
                let time = now
                    .duration_since(input.last_change_time)
                    .as_secs_f32();
                if time < cursor.stop_time || cursor.display {
                    **span = if select.is_empty() {
                        "|".to_string()
                    } else { 
                        if style.password_style {
                            format!("|{}|",change_passwd(select))
                        } else {
                            format!("|{select}|")
                        }
                    };
                    break;
                }
            }
            if span.0 != select {
                **span = if style.password_style {change_passwd(select.clone())} else { select.clone() };
            }
            break;
        }
    }
}
