// Native helper bodies retained from the pinned archive probe.
// Drawing, focus, scrolling, localization and file IO are boundaries, not engine proof.
enum Align { LeftIn, Middle }
enum AnchorTextInputMode { LocalKey, Raw }
globalvar ANCHOR;
enum Menu { Textbox, Settings, InfoToasts }
#macro spr_ui_journal_settings_icon_gameplay 100
global.menu = undefined;
global.warning_count = 0;
global.warnings = [];
global.notices = [];
global.hotkey_count = 0;
global.native_settings_writes = 0;
global.write_count = 0;
global.config_write_count = 0;
global.definitions_reordered = false;
global.write_mode = "ok";
global.read_mode = "ok";
global.disk = {};
global.actors = [];
global.free_count = 0;
function local_get(key) { return key; }
function num_display(x) { return string(x); }
function warn(a,b,c) { assert(false); }
function function_execute_alt(fn,args) { return method_call(fn,args ?? []); }
function TextProbe() constructor {
 self.local_key=undefined; self.text=""; self.text_input_mode=AnchorTextInputMode.Raw;
 function update_display_text() {}
 function set_align(x,y) {return self;}
 function set_x(x) {return self;}
static set_key = function(key) {
        if self.local_key == key && self.text_input_mode == AnchorTextInputMode.LocalKey {
            return self;
        }
        self.local_key = key;

        self.set_text(local_get(key));
        self.text_input_mode = AnchorTextInputMode.LocalKey;
        return self;
    }
static set_text = function(str) {
        if is_int64(str) || is_real(str) {
            str = num_display(str);
        } else if !is_string(str) {
            str = string(str);
        }
        if self.text == str {
            return self;
        }
        self.cache_is_dirty = true;
        self.text_input_mode = AnchorTextInputMode.Raw;
        self.text = str;
        self.update_display_text();
        return self;
    }
}
function PilotProbe() constructor {
 self.nodes=[];self.reset_count=0;
 function add(n) {array_push(self.nodes,n);}
 function request_newline() {}
 function reset() {self.nodes=[]; self.reset_count+=1;}
}
function NodeProbe() constructor {
 self.event_callbacks={tap:undefined};self.text_label=undefined;
static set_tap_callback = function(func, arg_array, intentional_override=false) {
        if self.event_callbacks.tap != undefined && !intentional_override {
            warn("{AnchorNode}'s tap callback is being overwritten at {}!", self, debug_get_callstack(2)[1]);
        }
        self.listens_for_hovers = true;
        self.listens_for_taps = true;
        self.run_logic = true;
        self.event_callbacks.tap = {
            func: func,
            arg_array: arg_array ?? [],
        }
        return self;
    }
static add_to_pilot = function(pilot, newline_after=false) {
        //
        self.pilot = pilot;
        self.pilot.add(self);
        if newline_after {
            self.pilot.request_newline();
        }
        return self;
    }
 function set_selected_getter(fn,args) {self.selected_fn=fn;self.selected_args=args;return self;}
 function is_selected() {return function_execute_alt(self.selected_fn,self.selected_args);}
 function add_text_label(key,a,b,c,d,e,f,g) {self.text_label=new TextProbe();self.text_label.set_key(key);return self;}
 function listen_for_hovers() {return self;}
 function set_sprite(s) {return self;}
 function set_align(x,y) {return self;}
 function set_x(x) {return self;}
 function tap() {function_execute_alt(self.event_callbacks.tap.func,self.event_callbacks.tap.arg_array);}
}
function ScrollerProbe() constructor {
 self.nodes=[];
 function new_element(height) {var n=new NodeProbe();array_push(self.nodes,n);return n;}
 function free() {global.free_count+=1;}
 function subscribe_to_pilot(pilot) {self.subscribed=pilot;}
}
function create_scroller(parent) {return new ScrollerProbe();}
ANCHOR={get_menu:function(kind){
 if(kind==Menu.InfoToasts) return {create_notification:function(key){array_push(global.notices,key);}};
 return global.menu;
},sprite:function(element){return new NodeProbe();},set_active_pilot:function(p){self.active=p;},wrap_for_local:function(raw){return "RAW:"+raw;}};
function SettingsProbe() constructor {
 self.active_page=undefined;self.option_scroller=undefined;self.category_scroller=new ScrollerProbe();
 self.option_pilot=new PilotProbe();self.category_pilot=new PilotProbe();self.journal={right_full_body:{}};
function create_category(name_key, open_callback, icon_sprite) {
        icon_sprite = icon_sprite == undefined ? string_to_asset(fmt("spr_ui_journal_settings_icon_{}", name_key)) : icon_sprite;
        var element = self.category_scroller.new_element(24)
            .set_tap_callback(function(name_key, open_callback) {
                self.active_page = name_key;
                if self.option_scroller != undefined {
                    self.option_scroller.free();
                }
                self.option_pilot.reset();
                self.option_scroller = create_scroller(self.journal.right_full_body);
                self.option_scroller.subscribe_to_pilot(self.option_pilot);
                open_callback();
                ANCHOR.set_active_pilot(self.option_pilot);
            }, [name_key, open_callback])
            .set_selected_getter(function(name_key) {
                return self.active_page == name_key;
            }, [name_key])
            .add_to_pilot(self.category_pilot, true)
            .add_text_label("misc_local/" + name_key)
        element.text_label
            .set_align(Align.LeftIn, Align.Middle)
            .set_x(23)

        ANCHOR.sprite(element)
            .set_sprite(icon_sprite)
            .set_align(Align.LeftIn, Align.Middle)
            .set_x(3)
        return element;
    }
function element(label_key, height, accept_inputs=true, font) {
        height = height == undefined ? 33 : height;

        var element = self.option_scroller.new_element(height)
            .add_text_label( //
                label_key,
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
                font,
            );

        if accept_inputs {
            element.listen_for_hovers();
            element.add_to_pilot(self.option_pilot, true);
        }

        element.text_label
            .set_align(Align.LeftIn, Align.Middle)
            .set_x(7)
        return element;
    }
}

function save_settings() { global.native_settings_writes++; assert(false, "Palette actions must not save native settings"); }
function mmapi_mod_declare(name, version) {}
function mmapi_register(callback) { global.initialize=callback; }
function mmapi_on(event_name, callback) { global.menu_opened=callback; }
function mmapi_hotkey_vk_from_name(name) { return name; }
function mmapi_hotkey_register(key, callback) { global.hotkey_count++; }
function mmapi_log_info(name, message) {}
function mmapi_log_warn(name, message) { global.warning_count++; array_push(global.warnings,message); }
function try_string_to_asset(name) {
 var start=string_pos("spr_npc_",name)==1 ? 9 : 2;
 return real(string_copy(name,start,string_length(name)-start+1));
}
function try_string_to_npc_id(name) {
 var definitions=lns_palette_definitions();
 for(var i=0;i<array_length(definitions);i++) if(definitions[i][0]==name) return i;
 return undefined;
}
function npc_id_to_gm_obj_id(id) { return 1000+id; }
function instance_number(object) { return object==1000 ? array_length(global.actors) : 0; }
function instance_find(object,index) { assert(object==1000); return global.actors[index]; }
function TestPortrait() constructor {
 sprite=10;index=1.75;
 function get_sprite(){return self.sprite;}
 function set_sprite(value){self.sprite=value;self.index=0;return self;}
 function set_index(value){self.index=value;return self;}
}
function TestTextbox() constructor {
 portrait=new TestPortrait();
 function set_speaker(speaker){self.portrait.set_sprite(speaker.sprite).set_index(speaker.index);}
}
function TestActor() constructor {
 animator={current:{sprite:300,index:2,frame:7.75},sprite:function(){return self.current.sprite;}};
 sprite_index=300; image_index=2; image_xscale=-1; x=34; y=72;
}
// An in-memory JSON/file boundary. No engine write or rename claim follows from it.
function mmapi_config_path(name) { assert(name=="lns_palette"); return "isolated/mod_data/lns_palette/lns_palette.json"; }
function file_exists(path) { return global.disk[$ path]!=undefined; }
function try_read_json_file(path,unused,decompress) {
 if(global.read_mode=="throw") throw "read denied";
 var content=global.disk[$ path];
 if(content==undefined) return undefined;
 try {return json_parse(content);} catch(err) {return undefined;}
}
function save_json_file(path,cfg) {
 global.write_count++;
 if(global.write_mode=="throw") throw "write denied";
 if(global.write_mode=="noop") return;
 if(global.write_mode=="truncate" && path==mmapi_config_path("lns_palette")) {global.disk[$ path]="{";return;}
 if(global.write_mode=="wrong" && path==mmapi_config_path("lns_palette")) {
  var wrong=json_parse(json_stringify(cfg));
  wrong.selections[$ lns_palette_definitions()[0][0]]="vanilla";
  global.disk[$ path]=json_stringify(wrong);return;
 }
 global.disk[$ path]=json_stringify(cfg);
}
// The following helper bodies match pinned MOMI v0.16.4, commit 2a55ec34.
function mmapi_config_dir(name) {}
function mmapi_warn_rate_limited(key,name,message) {mmapi_log_warn(name,message);}
function __mmapi_backup_last_good(mod_name, path) {
    try {
        if (file_exists(path) == false) { return; }
        var old_data = try_read_json_file(path, undefined, false);
        // The primary is already corrupt, so a copy of it would overwrite the
        // last good .bak there is with garbage.
        if (old_data == undefined) { return; }
        save_json_file(path + ".bak", old_data);
    } catch (err) {
        mmapi_warn_rate_limited(
            "backup:" + string(path), mod_name,
            "mmapi: could not write a last-good copy of " + string(path) + ": "
            + string(err) + ". The primary write is unaffected");
    }
}
function __mmapi_recover_last_good(mod_name, path) {
    try {
        var backup = string(path) + ".bak";
        if (file_exists(backup) == false) { return undefined; }
        var data = try_read_json_file(backup, undefined, false);
        if (data == undefined) {
            mmapi_log_warn(mod_name, "mmapi: " + string(path) + " did not parse, and "
                + "neither did " + backup + " - this data is lost");
            return undefined;
        }
        mmapi_log_warn(mod_name, "mmapi: " + string(path) + " did not parse; recovered "
            + "the last-good copy from " + backup);
        return data;
    } catch (err) {
        return undefined;
    }
}
function mmapi_config_load(mod_name) {
    var path = mmapi_config_path(mod_name);
    if (file_exists(path)) {
        var data = try_read_json_file(path, undefined, false);
        if (data != undefined) {
            return data;
        }
        // The primary is corrupt. Try the last-good copy before giving up.
        var recovered = __mmapi_recover_last_good(mod_name, path);
        if (recovered != undefined) {
            return recovered;
        }
        mmapi_log_warn(mod_name, "config exists but failed to parse: " + path);
    }
    return {};
}
function mmapi_config_save(mod_name, cfg) {
    try {
        mmapi_config_dir(mod_name);
        var path = mmapi_config_path(mod_name);
        // Last-good copy first, from the old contents. See
        // __mmapi_backup_last_good for why the ordering is the design.
        __mmapi_backup_last_good(mod_name, path);
        save_json_file(path, cfg);
    } catch (err) {
        mmapi_log_warn(mod_name, "config save failed: " + mmapi_config_path(mod_name));
    }
}
function mmapi_config_version_ok(loaded, version) {
    if (!is_struct(loaded)) { return false; }
    var v = loaded[$ "__config_version"];
    if (typeof(v) != "int64" && typeof(v) != "number") { return false; }
    return v == version;
}
function mmapi_config_read_valid(mod_name, version) {
    var loaded = mmapi_config_load(mod_name);   // {} if missing/unparseable
    if (mmapi_config_version_ok(loaded, version)) { return loaded; }
    return {};
}
function mmapi_config_write(mod_name, version, cfg) {
    global.config_write_count++; // Test spy; remaining body is unchanged.
    cfg.__config_version = version;
    mmapi_config_save(mod_name, cfg);
}
function mmapi_config_bool(container, key, dflt) {
    if (!is_struct(container)) { return dflt; }
    if (typeof(container[$ key]) == "bool") { return container[$ key]; }
    return dflt;
}
