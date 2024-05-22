use engine::{
    query,
    rigid_body::{DragSystem, GravitySystem, RigidBody, VelocitySystem},
    spawn, Collider, CollisionSystem, Component, System,
};

use crate::{
    hud::HudSystem,
    hurtbox::{Hitbox, Hurtbox, HurtboxSystem, Victim},
    keyset::Keyset,
    knockoff::KnockoffSystem,
    player::{Player, PlayerKind},
    player_attack::{PlayerAttack, PlayerAttackSystem},
    player_movement::{PlayerMovement, PlayerMovementSystem},
    sprite_renderer::{Sprite, SpriteRenderer},
};

pub struct GameSystem(pub u64);

#[derive(Component, Clone)]
pub struct HeroesOnBoard {
    pub hero_1: shared::Hero,
    pub hero_2: shared::Hero,
}

impl System for GameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.add_system(CollisionSystem);
        ctx.add_system(VelocitySystem);
        ctx.add_system(SpriteRenderer);
        ctx.add_system(PlayerMovementSystem);
        ctx.add_system(GravitySystem);
        ctx.add_system(DragSystem);
        ctx.add_system(HurtboxSystem);
        ctx.add_system(KnockoffSystem);
        ctx.add_system(PlayerAttackSystem);
        ctx.add_system(HudSystem);
        ctx.add_system(DebugDrawer);

        let background = ctx.load_texture("textures/literally_dprk.png").unwrap();
        let nope = ctx.load_texture("textures/nuh-uh.png").unwrap();

        spawn!(
            ctx,
            Sprite::new(background).layer(2),
            RigidBody {
                rect: (1280.0, 720.0),
                ..Default::default()
            },
        );

        self.spawn_player(ctx, (400.0, 200.0), Keyset::Wasd, PlayerKind::Left);

        self.spawn_player(ctx, (600.0, 200.0), Keyset::ArrowKeys, PlayerKind::Right);

        spawn!(
            ctx,
            RigidBody {
                pos: (250.0, 200.0),
                rect: (32.0, 32.0),
                ..Default::default()
            },
            Collider::new(),
            Sprite::new(nope),
        );

        spawn!(
            ctx,
            RigidBody {
                pos: (900.0, 400.0),
                rect: (32.0, 32.0),
                ..Default::default()
            },
            Collider::new(),
            Sprite::new(nope),
        );

        spawn!(
            ctx,
            RigidBody {
                pos: (184.0, 540.0),
                rect: (960.0, 128.0),
                ..Default::default()
            },
            Collider::new(),
        );

        Ok(())
    }

    fn on_update(&self, _ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}

impl GameSystem {
    fn spawn_player(
        &self,
        ctx: &mut engine::Context,
        pos: (f64, f64),
        keyset: Keyset,
        kind: PlayerKind,
    ) {
        let scale = 1.5;
        let pixel_ratio = 4.0;

        let heroes = ctx.clone_one::<HeroesOnBoard>();
        let hero = match kind {
            PlayerKind::Left => heroes.hero_1,
            PlayerKind::Right => heroes.hero_2,
        };

        let texture = {
            let path = crate::hero_info::HeroInfo::from(&hero.kind).texture_path;
            ctx.load_texture(path).unwrap()
        };
        let size = match hero.kind {
            shared::HeroKind::Centrist => (20.0, 28.0),
            shared::HeroKind::Strong => (24.0, 32.0),
            shared::HeroKind::Speed => (20.0, 29.0),
            shared::HeroKind::Tankie => (22.0, 28.0),
        };
        let sprite_offset = match hero.kind {
            shared::HeroKind::Centrist => (-4.0, -4.0),
            shared::HeroKind::Strong => (-4.0, 0.0),
            shared::HeroKind::Speed => (-6.0, -3.0),
            shared::HeroKind::Tankie => (-5.0, -4.0),
        };
        let hitbox_size = (16.0, 24.0);
        let hitbox_offset = (
            (size.0 - hitbox_size.0) / 2.0,
            (size.1 - hitbox_size.1) - 1.0,
        );
        spawn!(
            ctx,
            Sprite::new(texture)
                .layer(1)
                .size((32.0 * pixel_ratio * scale, 32.0 * pixel_ratio * scale))
                .offset((
                    sprite_offset.0 * pixel_ratio * scale,
                    sprite_offset.1 * pixel_ratio * scale
                )),
            Hitbox {
                size: (
                    hitbox_size.0 * pixel_ratio * scale,
                    hitbox_size.1 * pixel_ratio * scale
                ),
                offset: (
                    hitbox_offset.0 * pixel_ratio * scale,
                    hitbox_offset.1 * pixel_ratio * scale
                )
            },
            RigidBody {
                pos,
                rect: (size.0 * pixel_ratio * scale, size.1 * pixel_ratio * scale),
                gravity: true,
                drag: true,
                ..Default::default()
            },
            Collider::new().resolving(),
            PlayerMovement::new(keyset.clone()),
            Player {
                kind,
                hero,
                knockback_modifier: 0.0,
                lives: 3,
            },
            PlayerAttack::new(keyset, 0.0),
            Victim::default()
        );
    }
}

struct DebugDrawer(pub u64);

impl System for DebugDrawer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider) {
            let body = ctx.select::<RigidBody>(id).clone();
            self.draw_outline(ctx, body.pos, body.rect, 2.0, (0, 125, 255))?;
        }
        for id in query!(ctx, RigidBody, Hurtbox) {
            let body = ctx.select::<RigidBody>(id).clone();
            self.draw_outline(ctx, body.pos, body.rect, 2.0, (255, 0, 0))?;
        }
        for id in query!(ctx, RigidBody, Hitbox) {
            let body = ctx.select::<RigidBody>(id).clone();
            let hitbox = ctx.select::<Hitbox>(id).clone();
            self.draw_outline(
                ctx,
                (body.pos.0 + hitbox.offset.0, body.pos.1 + hitbox.offset.1),
                hitbox.size,
                2.0,
                (0, 255, 125),
            )?;
        }
        Ok(())
    }
}

impl DebugDrawer {
    fn draw_outline(
        &self,
        ctx: &mut engine::Context,
        pos: (f64, f64),
        size: (f64, f64),
        width: f64,
        color: (u8, u8, u8),
    ) -> Result<(), engine::Error> {
        ctx.draw_rect(
            color,
            pos.0 as i32,
            pos.1 as i32,
            size.0 as u32,
            width as u32,
        )?;
        ctx.draw_rect(
            color,
            (pos.0 + size.0 - width) as i32,
            pos.1 as i32,
            width as u32,
            size.1 as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.0 as i32,
            pos.1 as i32,
            width as u32,
            size.1 as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.0 as i32,
            (pos.1 + size.1 - width) as i32,
            size.0 as u32,
            width as u32,
        )?;
        Ok(())
    }
}
