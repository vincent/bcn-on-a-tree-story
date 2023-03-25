use yew::{Properties, Callback, function_component, html, Html, use_state};
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GeoTrackerProps {
    pub on_coords_change: Callback<(f64, f64)>,
}

#[function_component(Geolocation)]
pub fn geolocation(
    GeoTrackerProps {
        on_coords_change,
    }: &GeoTrackerProps,
) -> Html {
    let geo = use_geolocation();
    let state = use_state(|| 0.0);

    let throttle = {
        let geo = geo.clone();
        let on_coords_change = on_coords_change.clone();
        use_throttle(
            move || {
                let last = state.clone();
                if *last != (geo.latitude + geo.longitude) {
                    on_coords_change.emit((geo.latitude, geo.longitude));
                }
                last.set(geo.latitude + geo.longitude);
            },
            2000,
        )
    };

    throttle.run();

    html! {
        <div class="waiting-geo">
            // if geo.loading {
            //     { "waiting for geolocation... please ?" }
            // } else {
            //     <small>
            //         // { geo.latitude }{ " : " } { geo.longitude }
            //     </small>
            // }
        </div>
    }
}
