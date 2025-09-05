use mlua::prelude::*;

use dom_smoothie;

pub struct Readability(dom_smoothie::Readability);


impl LuaUserData for Readability {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method("get_article_title", |_, this, ()| {
            let title= this.0.get_article_title();
            Ok(title.to_string())
        });
    }
}


#[mlua::lua_module(name = "dom_smoothie")]
fn dom_smoothie_lua(lua: &'_ Lua) -> LuaResult<LuaTable<'_>> {
    let exports = lua.create_table()?;

    let readability_ctor = lua.create_function(|_, (html, doc_url, config): (String, Option<String>, Option<LuaTable>)| {
    let mut cfg = dom_smoothie::Config::default();

    if let Some(opts) = config {
        if let Ok(v) = opts.get("keep_classes") { cfg.keep_classes = v; }
        if let Ok(v) = opts.get("classes_to_preserve") { cfg.classes_to_preserve = v; }
        if let Ok(v) = opts.get("max_elements_to_parse") { cfg.max_elements_to_parse = v; }
        if let Ok(v) = opts.get("disable_json_ld") { cfg.disable_json_ld = v; }
        if let Ok(v) = opts.get("n_top_candidates") { cfg.n_top_candidates = v; }
        if let Ok(v) = opts.get("char_threshold") { cfg.char_threshold = v; }
        if let Ok(v) = opts.get("min_score_to_adjust") { cfg.min_score_to_adjust = v; }
        if let Ok(v) = opts.get("readable_min_score") { cfg.readable_min_score = v; }
        if let Ok(v) = opts.get("readable_min_content_length") { cfg.readable_min_content_length = v; }
        //if let Ok(v) = opts.get("candidate_select_mode") { cfg.candidate_select_mode = v; }
        //if let Ok(v) = opts.get("text_mode") { cfg.text_mode = v; }
    }

    

    let readability = dom_smoothie::Readability::new(html, doc_url.as_deref(), Some(cfg))
        .map_err(|e| LuaError::external(e))?;
    Ok(Readability(readability))

})?;

    exports.set("Readability", readability_ctor)?;
    Ok(exports)
}