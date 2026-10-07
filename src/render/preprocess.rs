//! Preprocessing and pre-rendering with KaTeX.
use katex::KatexContext;

use super::*;

/// Render all Katex equations.
pub fn process_all_chapters_prerender(
    book: &mut Book,
    cfg: &KatexConfig,
    stylesheet_header: &str,
    ctx: &PreprocessorContext,
) {
    let extra_opts = cfg.build_extra_opts();
    let macros = cfg.load_macros(&ctx.root);
    let katex_ctx = KatexContext::default();

    book.chapters_mut_thin()
        .into_par_iter()
        .filter(|chapter| !chapter.is_draft_chapter())
        .for_each(|chapter| {
            *chapter.content = process_chapter_prerender(
                chapter.content,
                &katex_ctx,
                cfg,
                &macros,
                stylesheet_header,
                &extra_opts,
            );
        });
}

/// Render Katex equations in a `Chapter` as HTML, and add the Katex CSS.
pub fn process_chapter_prerender(
    raw_content: &str,
    katex_ctx: &KatexContext,
    cfg: &KatexConfig,
    macros: &HashMap<String, String>,
    stylesheet_header: &str,
    extra_opts: &ExtraOpts,
) -> String {
    get_render_tasks(raw_content, stylesheet_header, extra_opts)
        .into_par_iter()
        .map_init(
            || cfg.build_opts_from_macros(macros),
            |(inline_opts, display_opts), rend| match rend {
                Render::Text(t) => t.into(),
                Render::InlineTask(item) => {
                    render(item, katex_ctx, inline_opts, extra_opts, false).into()
                }
                Render::DisplayTask(item) => {
                    render(item, katex_ctx, display_opts, extra_opts, true).into()
                }
            },
        )
        .collect::<Vec<Cow<_>>>()
        .join("")
}
