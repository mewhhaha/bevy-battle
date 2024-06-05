use bevy::{prelude::*, render::view::RenderLayers};

use crate::{helpers::LAYER_INTERACTIVE, Interactive, Player};

#[derive(Default, Component)]
pub struct RangeInteraction(pub f32);

fn player_set_closest_interactive(
    query: Query<(&Transform, &RangeInteraction), With<Player>>,
    mut interactive_items: Query<(Entity, &Transform, &mut Interactive)>,
) {
    let (player_transform, &RangeInteraction(range)) = query.single();

    for (_, _, mut interactive) in interactive_items.iter_mut() {
        interactive.0 = false;
    }

    let closest_entity = interactive_items
        .iter()
        .map(|(entity, transform, _)| {
            (
                entity,
                player_transform.translation.distance(transform.translation),
            )
        })
        .filter(|(_, distance)| *distance < range)
        .min_by(|(_, fst), (_, snd)| fst.total_cmp(snd))
        .map(|(entity, _)| entity);

    if let Some((_, _, mut interactive)) =
        closest_entity.and_then(|e| interactive_items.get_mut(e).ok())
    {
        interactive.0 = true;
    }
}

fn set_interactive_render_layer(
    mut commands: Commands,
    player_query: Query<(Entity, &RenderLayers), With<Player>>,
    query: Query<(Entity, &RenderLayers, &Interactive)>,
) {
    for (entity, render_layers, Interactive(interactive)) in query.iter() {
        let updated_layers = if *interactive {
            render_layers.with(LAYER_INTERACTIVE)
        } else {
            render_layers.without(LAYER_INTERACTIVE)
        };
        commands.entity(entity).insert(updated_layers);
    }

    let (player_entity, player_layers) = player_query.single();
    let interacting = query
        .iter()
        .any(|(_, _, Interactive(interactive))| *interactive);

    let mut player_commands = commands.entity(player_entity);

    if interacting {
        player_commands.insert(player_layers.with(LAYER_INTERACTIVE));
    } else {
        player_commands.insert(player_layers.without(LAYER_INTERACTIVE));
    }
}

pub struct InteractionOutlinePlugin;

impl Plugin for InteractionOutlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (player_set_closest_interactive, set_interactive_render_layer).chain(),
        );
    }
}
