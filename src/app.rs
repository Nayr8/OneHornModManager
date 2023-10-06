use std::rc::Rc;
use yew::prelude::*;
use crate::components::mod_list::ModList;
use crate::components::file_explorer::FileExplorer;
use crate::components::top_bar::TopBar;
use crate::components::bottom_bar::BottomBar;
use crate::components::console::Console;
use models::Mod;


#[function_component(App)]
pub fn app() -> Html {
    let mods: UseStateHandle<Rc<Vec<Mod>>> = use_state(|| Rc::new(vec![
        Mod {
            name: String::from("The Test Mod"),
            description: String::from("The Test mod for testing things")
        },
        Mod {
            name: String::from("The Test Mod 2"),
            description: String::from("The second Test mod for testing things")
        },
        Mod {
            name: String::from("The Test Mod 3"),
            description: String::from("The third Test mod for testing things")
        },
    ]));

    let file_explorer_open = use_state(|| false);
    let console_open = use_state(|| false);
    let selected_mod: UseStateHandle<Option<usize>> = use_state(|| None);

    html! {
        <div class="app">
            <TopBar
                file_explorer_open={file_explorer_open.clone()}
                selected_mod={selected_mod.clone()} />
            if *file_explorer_open {
                <FileExplorer
                    file_explorer_open={file_explorer_open.clone()} />
            } else {
                <ModList
                    mods={(*mods).clone()}
                    selected_mod={selected_mod.clone()} />
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