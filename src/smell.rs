use bevy::prelude::*;

#[derive(Component)]
pub struct EmitsSmell {
    pub smell: f32,
}

#[derive(Component, Debug, Default)]
pub struct CanSmell {
    pub smell_strength: f32,
    pub smell: f32,
    strongest_ever: f32,
}

#[derive(Bundle)]
pub struct CanSmellBundle {
    pub can_smell: CanSmell,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl CanSmellBundle {
    pub fn new(smell_strength: f32, transform: Transform, texture: Handle<Image>) -> Self {
        CanSmellBundle {
            can_smell: CanSmell {
                smell_strength,
                ..default()
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 1.),
                    custom_size: Some(Vec2::new(smell_strength / 6.0, smell_strength / 6.0)),
                    ..default()
                },
                transform,
                texture,
                ..default()
            },
        }
    }
}

pub fn smell_system(
    emitters: Query<(&EmitsSmell, &GlobalTransform)>,
    mut receivers: Query<(&mut CanSmell, &GlobalTransform)>,
) {
    for (mut receiver, receiver_transform) in receivers.iter_mut() {
        let mut new_smell = 0.;

        for (emitter, emitter_transform) in emitters.iter() {
            let distance = receiver_transform
                .translation
                .distance(emitter_transform.translation);

            if distance < receiver.smell_strength + emitter.smell {
                new_smell += distance / (receiver.smell_strength + emitter.smell);
            }
        }

        receiver.strongest_ever = new_smell.max(receiver.strongest_ever);

        // don't amplify 0 smell by dividing by 0
        receiver.smell = if receiver.strongest_ever <= 0.0001 {
            0.0
        } else {
            new_smell / receiver.strongest_ever
        };
    }
}
