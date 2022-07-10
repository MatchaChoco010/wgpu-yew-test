use gloo::render::request_animation_frame;
use std::rc::Rc;
use yew::prelude::*;

use crate::renderer::Renderer;

enum CanvasAction {
    Init(Renderer),
    Render(f64),
}

struct CanvasState {
    renderer: Option<Renderer>,
}
impl CanvasState {
    fn new() -> Self {
        Self { renderer: None }
    }
}
impl Reducible for CanvasState {
    type Action = CanvasAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CanvasAction::Init(renderer) => Rc::new(CanvasState {
                renderer: Some(renderer),
            }),
            CanvasAction::Render(delta_time) => {
                if let Some(renderer) = &self.renderer {
                    renderer.render(delta_time);
                }
                self
            }
        }
    }
}

#[function_component(TriangleCanvas)]
pub fn triangle_canvas() -> Html {
    let canvas_ref = use_node_ref();
    let reducer = use_reducer(CanvasState::new);
    use_effect_with_deps(
        {
            let reducer = reducer.clone();
            move |canvas_ref: &NodeRef| {
                if let Some(canvas) = canvas_ref.cast() {
                    reducer.dispatch(CanvasAction::Init(Renderer::init(canvas)));
                }
                || ()
            }
        },
        canvas_ref.clone(),
    );
    use_effect(move || {
        let handle = request_animation_frame(move |delta_time| {
            reducer.dispatch(CanvasAction::Render(delta_time));
        });
        move || drop(handle)
    });

    html! {
        <canvas ref={canvas_ref} width=400 height=300/>
    }
}
