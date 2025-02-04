//! Re-usable component

pub mod advisory;
pub mod async_state_renderer;
pub mod backend;
pub mod catalog;
pub mod common;
pub mod content;
pub mod cvss;
pub mod error;
pub mod package;
pub mod search;
pub mod severity;
pub mod simple_pagination;
pub mod spdx;
pub mod table_wrapper;

use std::ops::Deref;

use crate::components::error::Error;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_more_hooks::prelude::{UseAsyncHandleDeps, UseAsyncState};

#[function_component(ExtLinkIcon)]
pub fn ext_link_icon() -> Html {
    html!(<span class="pf-u-icon-color-light pf-u-ml-sm pf-u-font-size-sm">{ Icon::ExternalLinkAlt }</span>)
}

#[function_component(Trusted)]
pub fn trusted() -> Html {
    html!(<Label color={Color::Gold} label="Trusted"/>)
}

pub fn remote_content<T, E, FB>(fetch: &UseAsyncState<T, E>, body: FB) -> Html
where
    FB: FnOnce(&T) -> Html,
    E: std::error::Error,
{
    match &*fetch {
        UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
        UseAsyncState::Ready(Ok(data)) => body(data),
        UseAsyncState::Ready(Err(err)) => html!(<Error err={err.to_string()} />),
    }
}

pub fn remote_refs_count_title<T, E, F, R, X>(
    fetch: &UseAsyncHandleDeps<T, E>,
    f: F,
    singular: &str,
    plural: &str,
) -> String
where
    F: FnOnce(&T) -> Option<&R>,
    R: Deref<Target = [X]>,
{
    match &**fetch {
        UseAsyncState::Ready(Ok(data)) => match f(data).map(|r| r.len()) {
            Some(len) => count_title(len, singular, plural),
            None => plural.to_string(),
        },
        _ => plural.to_string(),
    }
}

pub fn count_title(len: usize, singular: &str, plural: &str) -> String {
    let s = match len {
        1 => singular,
        _ => plural,
    };

    format!("{len} {s}")
}
