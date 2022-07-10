use yew::prelude::*;
use yew_style_in_rs::*;

mod renderer;
mod triangle_canvas;
use triangle_canvas::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    style! {
        let css = css!{r#"
            width: 100%;
            min-height: 100%;
            padding: 16px;
            background: #101010;
            color: #ccc;
        "#};
    }
    html! {
        <div class={css}>
            <TriangleCanvas/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("app"));
    log::info!("Starting app");
    yew::start_app::<App>();
}
