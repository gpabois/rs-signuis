use leptos::{
    component, create_resource, create_signal, view, CollectView, ErrorBoundary, IntoView, Show,
    SignalGet as _, SignalUpdate, Suspense,
};
use leptos_leaflet::*;

use crate::api;
use signuis_core::forms::reporting::CreateNuisanceReportForm;

#[component]
pub fn NuisanceReportForm() -> impl IntoView {
    /// L'étape du formulaire.
    #[derive(Clone, Copy)]
    pub enum Step {
        ChooseNuisanceFamily,
        ChooseNuisanceType,
        ChooseIntensity,
        Summary,
    }

    impl Step {
        pub fn has_next(&self) -> bool {
            !matches!(self, Step::Summary)
        }

        pub fn has_prev(&self) -> bool {
            !matches!(self, Step::ChooseNuisanceFamily)
        }

        pub fn next(&mut self) {
            *self = match self {
                Step::ChooseNuisanceFamily => Step::ChooseNuisanceType,
                Step::ChooseNuisanceType => Step::ChooseIntensity,
                Step::ChooseIntensity => Step::Summary,
                Step::Summary => Step::Summary,
            }
        }

        pub fn prev(&mut self) {
            *self = match self {
                Step::ChooseNuisanceFamily => Step::ChooseNuisanceFamily,
                Step::ChooseNuisanceType => Step::ChooseNuisanceFamily,
                Step::ChooseIntensity => Step::ChooseNuisanceType,
                Step::Summary => Step::ChooseIntensity,
            }
        }
    }

    let (form, form_writer) = create_signal(CreateNuisanceReportForm::default());

    let (current_step, current_step_writer) = create_signal(Step::ChooseNuisanceFamily);
    let next_step_exists = move || current_step.get().has_next();
    let prev_step_exists = move || current_step.get().has_prev();
    let next_step = move || current_step_writer.update(Step::next);
    let prev_step = move || current_step_writer.update(Step::prev);

    let nuisance_families = create_resource(
        || (),
        |_| async move { api::reporting::list_nuisance_families().await },
    );
    view! {
        {move || {
            match current_step.get() {
                Step::ChooseNuisanceFamily => view! {
                    <div>
                        <h1>"Quelle famille de nuisance ?"</h1>
                        <Suspense>
                            <ErrorBoundary fallback=move|error| view!{{error.to_string()}}>
                                {move || nuisance_families.get().map(|nuisance_families| nuisance_families.map(|nuisance_families| view!{
                                    <select>
                                        {nuisance_families
                                            .into_iter()
                                            .map(|nuisance_family| view!{
                                                <option>{nuisance_family.label}</option>
                                            })
                                            .collect_view()
                                        }
                                    </select>
                                }))}
                            </ErrorBoundary>
                        </Suspense>
                    </div>
                }.into_any(),
                Step::ChooseNuisanceType => view!{
                    <h1>"Quelle est la nature de la nuissance ?"</h1>
                }.into_any(),
                Step::ChooseIntensity => view! {
                    <h1>"Quelle est l'intensité perçue ?"</h1>
                }.into_any(),
                Step::Summary => view!{
                    <h1>"En résumé..."</h1>
                }.into_any(),
            }
        }}
        <div class="flex gap-x-1">
            <Show when=prev_step_exists>
                <button class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    on:click=move |_| prev_step()>
                    Précédent
                </button>
            </Show>
            <Show when=next_step_exists>
                <button class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    on:click=move |_| next_step()>
                    Suivant
                </button>
            </Show>
        </div>

    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (display_nuisance_report_form, set_nuisance_report_form_display) =
        create_signal::<bool>(true);
    view! {
        <div class="w-screen h-screen bg-slate-500 flex flex-col">
            <div class="min-h-10 bg-slate-200 absolute top-4 z-10 w-full">
                "Menu"
            </div>
            <MapContainer
                class="w-full flex-auto z-0"
                center=Position::new(51.505, -0.09)
                zoom=13.0
                set_view=true>
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
            </MapContainer>
            <Show when=move || display_nuisance_report_form.get() fallback=||view!{} >
                <div class="bg-slate-100 h-200 p-4">
                    <NuisanceReportForm/>
                </div>
            </Show>
        </div>
    }
}
