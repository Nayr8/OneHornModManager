use yew::prelude::*;
use models::LogSeverity;
use crate::logger::Logger;

#[derive(Properties, PartialEq)]
pub struct ConsoleProps {

}

#[function_component(Console)]
pub fn console(_props: &ConsoleProps) -> Html {
    let log_lines = use_state(Vec::new);
    Logger::read_logs_into(log_lines.clone()); // TODO reorganise this

    let log_lines: Html = log_lines.iter().rev().map(|log_line| {
        let class = match log_line.severity {
            LogSeverity::Debug | LogSeverity::Trace => "console-debug",
            LogSeverity::Info => "console-info",
            LogSeverity::Warn => "console-warning",
            LogSeverity::Error | LogSeverity::Critical => "console-error",
        };
        html! {
            <div class={classes!(class)}>{log_line.to_string()}</div>
        }
    }).collect();

    html! {
        <div class="console">
            {log_lines}
        </div>
    }
}