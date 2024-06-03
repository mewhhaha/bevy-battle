use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ActiveElement(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct OverElement(pub Option<Entity>);

#[derive(Event)]
pub struct OnClick(pub Entity);

#[derive(Event)]
pub struct OnFocus(pub Entity);

#[derive(Event)]
pub struct OnBlur(pub Entity);

#[derive(Event)]
pub struct OnMouseEnter(pub Entity);

#[derive(Event)]
pub struct OnMouseLeave(pub Entity);

#[derive(Component)]
pub struct Focus;

#[derive(Component)]
pub struct Disabled;

#[derive(Component)]
pub struct Hover;

fn on_click(
    query: Query<(Entity, &Interaction), Without<Disabled>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_element: ResMut<ActiveElement>,
    mut on_click: EventWriter<OnClick>,
) {
    let just_pressed =
        keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);

    let clicked = query
        .iter()
        .find_map(|(entity, interaction)| {
            match (interaction, mouse.just_pressed(MouseButton::Left)) {
                (Interaction::Pressed, true) => Some(entity),
                _ => None,
            }
        })
        .or(if just_pressed { active_element.0 } else { None });

    if let Some(next) = clicked {
        on_click.send(OnClick(next));

        if active_element.0 != Some(next) {
            active_element.0 = Some(next);
        }
    }
}

fn on_hover(
    query: Query<(Entity, &Interaction), Without<Disabled>>,
    mut over_element: ResMut<OverElement>,
) {
    let hovered = query
        .iter()
        .find_map(|(entity, interaction)| match interaction {
            Interaction::Hovered => Some(entity),
            _ => None,
        });

    if let Some(next) = hovered {
        if over_element.0 != Some(next) {
            over_element.0 = Some(next);
        }
    }
}

fn on_mousehover_and_mouseout(
    mut commands: Commands,
    mut local: Local<OverElement>,
    focus: Res<OverElement>,
    mut mouse_enter_events: EventWriter<OnMouseEnter>,
    mut mouse_leave_events: EventWriter<OnMouseLeave>,
) {
    if local.0 != focus.0 {
        if let Some(prev) = local.0 {
            commands.entity(prev).remove::<Focus>();
            mouse_leave_events.send(OnMouseLeave(prev));
        }

        if let Some(next) = focus.0 {
            commands.entity(next).insert(Focus);
            mouse_enter_events.send(OnMouseEnter(next));
        }

        local.0 = focus.0;
    }
}

fn on_focus_and_blur(
    mut commands: Commands,
    mut local: Local<ActiveElement>,
    focus: Res<ActiveElement>,
    mut focus_events: EventWriter<OnFocus>,
    mut blur_events: EventWriter<OnBlur>,
) {
    if local.0 != focus.0 {
        if let Some(prev) = local.0 {
            commands.entity(prev).remove::<Focus>();
            blur_events.send(OnBlur(prev));
        }

        if let Some(next) = focus.0 {
            commands.entity(next).insert(Focus);
            focus_events.send(OnFocus(next));
        }

        local.0 = focus.0;
    }
}

pub struct UiEventsPlugin;

impl Plugin for UiEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                (on_click, on_focus_and_blur).chain(),
                (on_hover, on_mousehover_and_mouseout).chain(),
            ),
        )
        .insert_resource(ActiveElement::default())
        .insert_resource(OverElement::default())
        .add_event::<OnClick>()
        .add_event::<OnFocus>()
        .add_event::<OnBlur>()
        .add_event::<OnMouseEnter>()
        .add_event::<OnMouseLeave>();
    }
}
