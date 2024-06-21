use leptos::{component, create_signal, view, IntoView};
use leptos_leaflet::leaflet::{LocationEvent, Map};
use leptos_leaflet::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (marker_position, set_marker_position) = create_signal(Position::new(51.49, -0.08));
    let (map, set_map) = create_signal(None::<Map>);
    
    let events = MapEvents::new().location_found(move |loc: LocationEvent| {

    });
    view! {
        <MapContainer 
            style="height: 400px" 
            center=Position::new(51.505, -0.09) 
            zoom=13.0 set_view=true 
            map=set_map
            events 
            locate=true 
            watch=true>
            <TileLayer 
                url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" 
                attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
            />
        </MapContainer>
    }
}
