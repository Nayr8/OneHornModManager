use std::rc::Rc;
use yew::prelude::*;
use crate::pages::main_page::MainPage;
use crate::pages::file_explorer::FileExplorer;
use crate::top_bar::TopBar;
use crate::bottom_bar::BottomBar;
use crate::console::Console;
use models::{Mod, Status};

use crate::listeners;

#[function_component(App)]
pub fn app() -> Html {
    let mods: UseStateHandle<Status<Rc<Vec<Mod>>>> = use_state(|| Status::Loading);

    let file_explorer_open = use_state(|| false);
    let console_open = use_state(|| false);
    let selected_mod: UseStateHandle<Option<usize>> = use_state(|| None);

    let profile_open = use_state(|| false);
    let profile_create_new = use_state(|| false);

    let dropped_file = use_state(|| None);

    use_effect_with_deps(|(dropped_file, profile_open, profile_create_new, file_explorer_open)| {
        let profile_open = profile_open.clone();
        let profile_create_new = profile_create_new.clone();
        let file_explorer_open = file_explorer_open.clone();
        listeners::listen_for_file_drop(dropped_file.clone(), move || {
            profile_open.set(false);
            profile_create_new.set(false);
            file_explorer_open.set(true);
        });
    }, (dropped_file.clone(), profile_open.clone(), profile_create_new.clone(), file_explorer_open.clone()));

    html! {
        <div class="app">
            <TopBar
                file_explorer_open={file_explorer_open.clone()}
                selected_mod={selected_mod.clone()}
                mods={mods.clone()}
                profile_open={profile_open.clone()}
                profile_create_new={profile_create_new.clone()} />
            if *file_explorer_open {
                <FileExplorer
                    file_explorer_open={file_explorer_open.clone()}
                    selected_mod={selected_mod.clone()}
                    dropped_file={dropped_file.clone()} />
            } else {
                <MainPage
                    mods={mods.clone()}
                    selected_mod={selected_mod.clone()}
                    file_explorer_open={file_explorer_open.clone()}
                    profile_open={profile_open.clone()}
                    profile_create_new={profile_create_new.clone()} />
            }
            if *console_open {
                <Console />
            }
            <BottomBar console_open={console_open} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SimpleConsoleProps {
    message: UseStateHandle<String>,
}
#[function_component(SimpleConsole)]
pub fn simple_console(props: &SimpleConsoleProps) -> Html {
    html! {
        <div class="simple-console">{props.message.as_str()}</div>
    }
}