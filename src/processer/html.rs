use crate::action::Action;
use crate::role::RequestRole;
use crate::state::AppState;
use actix_web::web;
use lol_html::html_content::{ContentType, Element};
use lol_html::{element, errors::RewritingError, HtmlRewriter, Settings};
use std::error::Error;
use std::sync::atomic;

pub(crate) fn process_html(
    source: String,
    role: RequestRole,
    app_state: web::Data<AppState>,
) -> Result<String, RewritingError> {
    let content_handlers = match role {
        RequestRole::Bot => {
            let mut handlers = vec![
                element!("[ai-hole-bot]", |element| handler_bot(
                    element,
                    app_state.clone()
                )),
                element!("[ai-hole-human]", handler_void_human),
            ];

            for custom_selector in app_state.config.selectors.html.randomized.iter() {
                handlers.push(element!(custom_selector, |element| {
                    handler(element, Action::Randomize, app_state.clone());
                    Ok(())
                }));
            }
            for custom_selector in app_state.config.selectors.html.removed.iter() {
                handlers.push(element!(custom_selector, |element| {
                    handler(element, Action::Remove, app_state.clone());
                    Ok(())
                }));
            }

            handlers
        }
        RequestRole::Human => {
            vec![
                element!("[ai-hole-human]", |element| handler_human(
                    element,
                    app_state.clone()
                )),
                element!("[ai-hole-bot]", handler_void_bot),
            ]
        }
    };
    let settings = Settings {
        element_content_handlers: content_handlers,
        ..Default::default()
    };
    let mut output = Vec::new();
    let mut rewriter = HtmlRewriter::new(settings, |out: &[u8]| output.extend_from_slice(out));

    rewriter.write(source.as_bytes())?;
    rewriter.end()?;

    let output = String::from_utf8_lossy(&output).to_string();
    Ok(output)
}

fn handler_bot(
    element: &mut Element,
    app_state: web::Data<AppState>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let action: Option<Action> = element
        .get_attribute("ai-hole-bot")
        .expect("missing ai-hole-bot even though thats specified in the query")
        .as_str()
        .try_into()
        .ok();
    // Lets just not touch it if they used the wrong type, instead of failing the entire request
    // The attribute will stay there, so it will hopefully signal to them that they need to change?
    let Some(action) = action else {
        // Just in case
        element.set_attribute("ai-hole-error", "invalid action").expect("this is trusted input");
        return Ok(());
    };
    element.remove_attribute("ai-hole-bot");
    handler(element, action, app_state);
    Ok(())
}

fn handler_void_bot(element: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>> {
    element.remove_attribute("ai-hole-bot");
    Ok(())
}

fn handler_human(
    element: &mut Element,
    app_state: web::Data<AppState>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let action: Option<Action> = element
        .get_attribute("ai-hole-human")
        .expect("missing ai-hole-human even though thats specified in the query")
        .as_str()
        .try_into()
        .ok();
    // Lets just not touch it if they used the wrong type, instead of failing the entire request
    // The attribute will stay there, so it will hopefully signal to them that they need to change?
    let Some(action) = action else {
        // Just in case
        element.set_attribute("ai-hole-error", "invalid action").expect("this is trusted input");
        return Ok(());
    };
    element.remove_attribute("ai-hole-human");
    handler(element, action, app_state);
    Ok(())
}

fn handler_void_human(element: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>> {
    element.remove_attribute("ai-hole-human");
    Ok(())
}

fn handler(element: &mut Element, action: Action, app_state: web::Data<AppState>) {
    match action {
        Action::Remove => {
            // Stats
            app_state
                .stats
                .elements_removed
                .fetch_add(1, atomic::Ordering::Relaxed);

            // Removal
            element.remove()
        }
        Action::Randomize => {
            // Stats
            app_state
                .stats
                .elements_randomized
                .fetch_add(1, atomic::Ordering::Relaxed);

            // Randomization
            // TODO: Read from input text
            let word_count = element
                .get_attribute("ai-hole-randomize-word-count")
                .map(|s| s.parse::<usize>())
                .unwrap_or_else(|| Ok(app_state.config.default_random_words_count))
                .unwrap_or(app_state.config.default_random_words_count);

            element.remove_attribute("ai-hole-randomize-word-count");
            let text = app_state.generate_word_sequence(word_count);
            element.set_inner_content(&text, ContentType::Text);
        }
    }
}
