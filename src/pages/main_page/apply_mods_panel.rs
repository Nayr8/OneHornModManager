
use yew::prelude::*;
use crate::bindings::ModManager;
use crate::components::Button;
use crate::components::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::button::ButtonSize;


#[derive(Properties, PartialEq)]
pub struct ApplyModsPanelProps;
#[function_component(ApplyModsPanel)]
pub fn apply_mods_panel(_props: &ApplyModsPanelProps) -> Html {
    let applying = use_state(|| false);

    let apply = {
        let applying = applying.clone();
        move |_: MouseEvent| {
            ModManager::apply(applying.clone());
        }
    };

    html! {
        <div class="apply-mods-panel">
            <Button size={ButtonSize::Big} class="apply-mods-button" onclick={apply} disabled={*applying}>
                if *applying {
                    <Spinner size={SpinnerSize::Small} />
                } else {
                    {"Apply Mods"}
                }
            </Button>
        </div>
    }
}