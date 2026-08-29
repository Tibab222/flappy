use bevy::prelude::*;
use crate::{
    coin::Wallet,
    constants::{GameState, COLOR_HOVERED_BTN, COLOR_NORMAL_BTN, COLOR_PRESSED_BTN},
};

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoosterType {
    Shield,
}

#[derive(Resource, Default)]
pub struct ActiveBoosters {
    pub shield_charges: u32,
}

#[derive(Component)]
pub struct BuyButton(pub BoosterType, pub u32);

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct ShopUI;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveBoosters>()
            .add_systems(OnEnter(GameState::Shop), setup_shop_ui)
            .add_systems(OnExit(GameState::Shop), cleanup_shop_ui)
            .add_systems(
                Update,
                (handle_shop_interactions, handle_back_button).run_if(in_state(GameState::Shop)),
            );
    }
}

fn setup_shop_ui(mut commands: Commands, wallet: Res<Wallet>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            ShopUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("BOOSTER SHOP"),
                TextFont::from_font_size(40.0),
                TextColor(Color::srgb(1.0, 0.84, 0.0)),
            ));

            parent.spawn((
                Text::new(format!("Coins disponibles: {}", wallet.coins)),
                TextFont::from_font_size(20.0),
                TextColor(Color::WHITE),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                    BuyButton(BoosterType::Shield, 5),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Shield (3 pipes) - 5 Coins"),
                        TextFont::from_font_size(18.0),
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(COLOR_NORMAL_BTN),
                    BackButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("RETOUR"),
                        TextFont::from_font_size(22.0),
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn handle_shop_interactions(
    mut interaction_query: Query<
        (&Interaction, &BuyButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut wallet: ResMut<Wallet>,
    mut boosters: ResMut<ActiveBoosters>,
) {
    for (interaction, buy_btn, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let price = buy_btn.1;
                if wallet.coins >= price {
                    wallet.coins -= price;

                    match buy_btn.0 {
                        BoosterType::Shield => {
                            boosters.shield_charges += 3;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.7, 0.3));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
            }
        }
    }
}

fn handle_back_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = COLOR_PRESSED_BTN.into();
                next_state.set(GameState::Menu);
            }
            Interaction::Hovered => {
                *bg_color = COLOR_HOVERED_BTN.into();
            }
            Interaction::None => {
                *bg_color = COLOR_NORMAL_BTN.into();
            }
        }
    }
}

fn cleanup_shop_ui(mut commands: Commands, query: Query<Entity, With<ShopUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}