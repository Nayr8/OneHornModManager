use std::path::PathBuf;
use std::sync::Arc;
use yew::UseStateHandle;
use futures::StreamExt;
use yew::platform::spawn_local;
use crate::error;

pub fn listen_for_file_drop(dropped_file: UseStateHandle<Option<Arc<PathBuf>>>, on_file_dropped: impl Fn() + 'static) {
    spawn_local(async move {
        let mut tile_drop_stream = tauri_sys::event::listen::<Vec<Arc<PathBuf>>>("tauri://file-drop").await.unwrap();

        loop {
            while let Some(event) = tile_drop_stream.next().await {
                if event.payload.is_empty() { continue }
                if event.payload.len() != 1 {
                    error!("Uploading multiple files at th same time not supported found {}", event.payload.len());
                    continue;
                }
                dropped_file.set(Some(event.payload[0].clone()));
                on_file_dropped();
            }
        }
    });
}