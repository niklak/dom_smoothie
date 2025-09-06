use dom_smoothie;
use mlua::prelude::*;
use mlua::LuaSerdeExt;

pub struct Readability(dom_smoothie::Readability);

impl LuaUserData for Readability {
    fn add_fields<F: LuaUserDataFields<Self>>(_fields: &mut F) {}

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_article_title", |_, this, ()| {
            let title = this.0.get_article_title();
            Ok(title.to_string())
        });

        methods.add_method("parse_json_ld", |lua, this, ()| {
            lua.to_value(&this.0.parse_json_ld())
        });

        methods.add_method("get_article_metadata", |lua, this, json_ld: LuaValue| {
            let meta_ld: Option<dom_smoothie::Metadata> = lua.from_value(json_ld)?;
            lua.to_value(&this.0.get_article_metadata(meta_ld))
        });

        methods.add_method("is_probably_readable", |_, this, ()| {
            Ok(this.0.is_probably_readable())
        });

        methods.add_method_mut("parse", |lua, this, ()| {
            let article = this.0.parse().map_err(|e| LuaError::external(e))?;
            lua.to_value(&article)
        });

        methods.add_method_mut("parse_with_policy", |lua, this, policy: LuaValue| {
            let parse_policy: dom_smoothie::ParsePolicy = lua.from_value(policy)?;
            let article = this
                .0
                .parse_with_policy(parse_policy)
                .map_err(|e| LuaError::external(e))?;
            lua.to_value(&article)
        });
    }
}

#[mlua::lua_module(name = "dom_smoothie")]
fn dom_smoothie_lua(lua: &'_ Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let readability_ctor = lua.create_function(
        |lua_vm, (html, doc_url, config): (String, Option<String>, Option<LuaTable>)| {
            let mut cfg = dom_smoothie::Config::default();

            if let Some(opts) = config {
                if let Ok(v) = opts.get("keep_classes") {
                    cfg.keep_classes = v;
                }
                if let Ok(v) = opts.get("classes_to_preserve") {
                    cfg.classes_to_preserve = v;
                }
                if let Ok(v) = opts.get("max_elements_to_parse") {
                    cfg.max_elements_to_parse = v;
                }
                if let Ok(v) = opts.get("disable_json_ld") {
                    cfg.disable_json_ld = v;
                }
                if let Ok(v) = opts.get("n_top_candidates") {
                    cfg.n_top_candidates = v;
                }
                if let Ok(v) = opts.get("char_threshold") {
                    cfg.char_threshold = v;
                }
                if let Ok(v) = opts.get("min_score_to_adjust") {
                    cfg.min_score_to_adjust = v;
                }
                if let Ok(v) = opts.get("readable_min_score") {
                    cfg.readable_min_score = v;
                }
                if let Ok(v) = opts.get("readable_min_content_length") {
                    cfg.readable_min_content_length = v;
                }
                if let Ok(v) = opts.get("candidate_select_mode") {
                    cfg.candidate_select_mode = lua_vm.from_value(v)?;
                }
                if let Ok(v) = opts.get("text_mode") {
                    cfg.text_mode = lua_vm.from_value(v)?;
                }
            }

            let readability = dom_smoothie::Readability::new(html, doc_url.as_deref(), Some(cfg))
                .map_err(|e| LuaError::external(e))?;
            Ok(Readability(readability))
        },
    )?;

    exports.set("Readability", readability_ctor)?;
    Ok(exports)
}
