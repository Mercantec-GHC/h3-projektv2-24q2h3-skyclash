use std::rc::Rc;
use std::sync::Mutex;

use crate::ui2;
use engine::{query, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct MyMenu {
    dom: Rc<Mutex<ui2::Dom>>,
}

pub struct MyMenuSystem(pub u64);
impl System for MyMenuSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::constructors::*;

        let mut dom = ui2::Dom::new(
            Vert([
                Rect().with_height(100),
                Text("hello")
                    .with_padding(15)
                    .with_border_thickness(2)
                    .with_border_color((255, 255, 255)),
                Hori([
                    Text("world"),
                    Text(":3").with_id(34).with_background_color((0, 255, 255)),
                ])
                .with_on_click(12)
                .with_color((255, 150, 150)),
            ])
            .with_width(1280)
            .with_height(720)
            .with_background_color((0, 0, 0)),
        );

        dom.add_event_handler(12, |dom, _ctx, _node_id| {
            let Some(element) = dom.select_mut(34) else {
                return;
            };
            if let ui2::Kind::Text { text, .. } = &mut element.kind {
                *text = "some thing else".to_string();
            };
        });

        spawn!(
            ctx,
            MyMenu {
                dom: Rc::new(Mutex::new(dom))
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, MyMenu) {
            let my_menu = ctx.entity_component::<MyMenu>(id).clone();
            my_menu.dom.lock().unwrap().update(ctx);
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
