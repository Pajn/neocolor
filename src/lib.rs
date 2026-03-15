use ignore::WalkBuilder;
use nvim_oxi::api::{self, opts::*, types::*, Buffer};
use nvim_oxi::{Dictionary, Function, Object, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

static CSS_VARIABLES: Lazy<Arc<RwLock<HashMap<String, (u8, u8, u8)>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
static CSS_SCAN_STARTED: AtomicBool = AtomicBool::new(false);
static SETUP_DONE: AtomicBool = AtomicBool::new(false);

static NAMED_COLORS: Lazy<HashMap<&'static str, (u8, u8, u8)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("aliceblue", (240, 248, 255));
    m.insert("antiquewhite", (250, 235, 215));
    m.insert("aqua", (0, 255, 255));
    m.insert("aquamarine", (127, 255, 212));
    m.insert("azure", (240, 255, 255));
    m.insert("beige", (245, 245, 220));
    m.insert("bisque", (255, 228, 196));
    m.insert("black", (0, 0, 0));
    m.insert("blanchedalmond", (255, 235, 205));
    m.insert("blue", (0, 0, 255));
    m.insert("blueviolet", (138, 43, 226));
    m.insert("brown", (165, 42, 42));
    m.insert("burlywood", (222, 184, 135));
    m.insert("cadetblue", (95, 158, 160));
    m.insert("chartreuse", (127, 255, 0));
    m.insert("chocolate", (210, 105, 30));
    m.insert("coral", (255, 127, 80));
    m.insert("cornflowerblue", (100, 149, 237));
    m.insert("cornsilk", (255, 248, 220));
    m.insert("crimson", (220, 20, 60));
    m.insert("cyan", (0, 255, 255));
    m.insert("darkblue", (0, 0, 139));
    m.insert("darkcyan", (0, 139, 139));
    m.insert("darkgoldenrod", (184, 134, 11));
    m.insert("darkgray", (169, 169, 169));
    m.insert("darkgreen", (0, 100, 0));
    m.insert("darkgrey", (169, 169, 169));
    m.insert("darkkhaki", (189, 183, 107));
    m.insert("darkmagenta", (139, 0, 139));
    m.insert("darkolivegreen", (85, 107, 47));
    m.insert("darkorange", (255, 140, 0));
    m.insert("darkorchid", (153, 50, 204));
    m.insert("darkred", (139, 0, 0));
    m.insert("darksalmon", (233, 150, 122));
    m.insert("darkseagreen", (143, 188, 143));
    m.insert("darkslateblue", (72, 61, 139));
    m.insert("darkslategray", (47, 79, 79));
    m.insert("darkslategrey", (47, 79, 79));
    m.insert("darkturquoise", (0, 206, 209));
    m.insert("darkviolet", (148, 0, 211));
    m.insert("deeppink", (255, 20, 147));
    m.insert("deepskyblue", (0, 191, 255));
    m.insert("dimgray", (105, 105, 105));
    m.insert("dimgrey", (105, 105, 105));
    m.insert("dodgerblue", (30, 144, 255));
    m.insert("firebrick", (178, 34, 34));
    m.insert("floralwhite", (255, 250, 240));
    m.insert("forestgreen", (34, 139, 34));
    m.insert("fuchsia", (255, 0, 255));
    m.insert("gainsboro", (220, 220, 220));
    m.insert("ghostwhite", (248, 248, 255));
    m.insert("gold", (255, 215, 0));
    m.insert("goldenrod", (218, 165, 32));
    m.insert("gray", (128, 128, 128));
    m.insert("green", (0, 128, 0));
    m.insert("greenyellow", (173, 255, 47));
    m.insert("grey", (128, 128, 128));
    m.insert("honeydew", (240, 255, 240));
    m.insert("hotpink", (255, 105, 180));
    m.insert("indianred", (205, 92, 92));
    m.insert("indigo", (75, 0, 130));
    m.insert("ivory", (255, 255, 240));
    m.insert("khaki", (240, 230, 140));
    m.insert("lavender", (230, 230, 250));
    m.insert("lavenderblush", (255, 240, 245));
    m.insert("lawngreen", (124, 252, 0));
    m.insert("lemonchiffon", (255, 250, 205));
    m.insert("lightblue", (173, 216, 230));
    m.insert("lightcoral", (240, 128, 128));
    m.insert("lightcyan", (224, 255, 255));
    m.insert("lightgoldenrodyellow", (250, 250, 210));
    m.insert("lightgray", (211, 211, 211));
    m.insert("lightgreen", (144, 238, 144));
    m.insert("lightgrey", (211, 211, 211));
    m.insert("lightpink", (255, 182, 193));
    m.insert("lightsalmon", (255, 160, 122));
    m.insert("lightseagreen", (32, 178, 170));
    m.insert("lightskyblue", (135, 206, 250));
    m.insert("lightslategray", (119, 136, 153));
    m.insert("lightslategrey", (119, 136, 153));
    m.insert("lightsteelblue", (176, 196, 222));
    m.insert("lightyellow", (255, 255, 224));
    m.insert("lime", (0, 255, 0));
    m.insert("limegreen", (50, 205, 50));
    m.insert("linen", (250, 240, 230));
    m.insert("magenta", (255, 0, 255));
    m.insert("maroon", (128, 0, 0));
    m.insert("mediumaquamarine", (102, 205, 170));
    m.insert("mediumblue", (0, 0, 205));
    m.insert("mediumorchid", (186, 85, 211));
    m.insert("mediumpurple", (147, 112, 219));
    m.insert("mediumseagreen", (60, 179, 113));
    m.insert("mediumslateblue", (123, 104, 238));
    m.insert("mediumspringgreen", (0, 250, 154));
    m.insert("mediumturquoise", (72, 209, 204));
    m.insert("mediumvioletred", (199, 21, 133));
    m.insert("midnightblue", (25, 25, 112));
    m.insert("mintcream", (245, 255, 250));
    m.insert("mistyrose", (255, 228, 225));
    m.insert("moccasin", (255, 228, 181));
    m.insert("navajowhite", (255, 222, 173));
    m.insert("navy", (0, 0, 128));
    m.insert("oldlace", (253, 245, 230));
    m.insert("olive", (128, 128, 0));
    m.insert("olivedrab", (107, 142, 35));
    m.insert("orange", (255, 165, 0));
    m.insert("orangered", (255, 69, 0));
    m.insert("orchid", (218, 112, 214));
    m.insert("palegoldenrod", (238, 232, 170));
    m.insert("palegreen", (152, 251, 152));
    m.insert("paleturquoise", (175, 238, 238));
    m.insert("palevioletred", (219, 112, 147));
    m.insert("papayawhip", (255, 239, 213));
    m.insert("peachpuff", (255, 218, 185));
    m.insert("peru", (205, 133, 63));
    m.insert("pink", (255, 192, 203));
    m.insert("plum", (221, 160, 221));
    m.insert("powderblue", (176, 224, 230));
    m.insert("purple", (128, 0, 128));
    m.insert("rebeccapurple", (102, 51, 153));
    m.insert("red", (255, 0, 0));
    m.insert("rosybrown", (188, 143, 143));
    m.insert("royalblue", (65, 105, 225));
    m.insert("saddlebrown", (139, 69, 19));
    m.insert("salmon", (250, 128, 114));
    m.insert("sandybrown", (244, 164, 96));
    m.insert("seagreen", (46, 139, 87));
    m.insert("seashell", (255, 245, 238));
    m.insert("sienna", (160, 82, 45));
    m.insert("silver", (192, 192, 192));
    m.insert("skyblue", (135, 206, 235));
    m.insert("slateblue", (106, 90, 205));
    m.insert("slategray", (112, 128, 144));
    m.insert("slategrey", (112, 128, 144));
    m.insert("snow", (255, 250, 250));
    m.insert("springgreen", (0, 255, 127));
    m.insert("steelblue", (70, 130, 180));
    m.insert("tan", (210, 180, 140));
    m.insert("teal", (0, 128, 128));
    m.insert("thistle", (216, 191, 216));
    m.insert("tomato", (255, 99, 71));
    m.insert("turquoise", (64, 224, 208));
    m.insert("violet", (238, 130, 238));
    m.insert("wheat", (245, 222, 179));
    m.insert("white", (255, 255, 255));
    m.insert("whitesmoke", (245, 245, 245));
    m.insert("yellow", (255, 255, 0));
    m.insert("yellowgreen", (154, 205, 50));
    m
});

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 3 || hex.len() == 4 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = h.rem_euclid(1.0);
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn parse_component(s: &str, max: f32) -> f32 {
    if s.ends_with('%') {
        let val: f32 = s.trim_end_matches('%').parse().unwrap_or(0.0);
        (val / 100.0) * max
    } else {
        let val: f32 = s.parse().unwrap_or(0.0);
        val
    }
}

fn parse_hue(s: &str) -> f32 {
    if s.ends_with("deg") {
        s.trim_end_matches("deg").parse().unwrap_or(0.0) / 360.0
    } else if s.ends_with("grad") {
        s.trim_end_matches("grad").parse().unwrap_or(0.0) / 400.0
    } else if s.ends_with("rad") {
        s.trim_end_matches("rad").parse().unwrap_or(0.0) / (2.0 * std::f32::consts::PI)
    } else if s.ends_with("turn") {
        s.trim_end_matches("turn").parse().unwrap_or(0.0)
    } else {
        s.parse().unwrap_or(0.0) / 360.0
    }
}

fn apply_hl(
    buf: &mut Buffer,
    ns_id: u32,
    line: usize,
    start: usize,
    end: usize,
    r: u8,
    g: u8,
    b: u8,
) {
    let hl_name = format!("NeoColor_{:02x}{:02x}{:02x}", r, g, b);
    let bg = format!("#{:02x}{:02x}{:02x}", r, g, b);
    let brightness = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0;
    let fg = if brightness > 0.5 {
        "#000000"
    } else {
        "#ffffff"
    };

    let mut hl_opts = SetHighlightOpts::builder();
    hl_opts.background(&bg);
    hl_opts.foreground(fg);
    let _ = api::set_hl(0, &hl_name, &hl_opts.build());

    let ext_opts = SetExtmarkOpts::builder()
        .hl_group(hl_name.as_str())
        .end_row(line)
        .end_col(end)
        .build();
    let _ = buf.set_extmark(ns_id, line, start, &ext_opts);
}

fn resolve_color_string(s: &str) -> Option<(u8, u8, u8)> {
    if let Some(m) = Regex::new(r"#([0-9a-fA-F]{3,8})").unwrap().captures(s) {
        return parse_hex(&m[1]);
    }
    let css_rgb_re = Regex::new(
        r"rgba?\(\s*([\d.%]+)\s*[, ]\s*([\d.%]+)\s*[, ]\s*([\d.%]+)(?:\s*[/,]\s*([\d.%]+))?\s*\)",
    )
    .unwrap();
    if let Some(caps) = css_rgb_re.captures(s) {
        let r = parse_component(&caps[1], 255.0) as u8;
        let g = parse_component(&caps[2], 255.0) as u8;
        let b = parse_component(&caps[3], 255.0) as u8;
        return Some((r, g, b));
    }
    let css_hsl_re = Regex::new(r"hsla?\(\s*([\d.deggradturn]+)\s*[, ]\s*([\d.%]+)\s*[, ]\s*([\d.%]+)(?:\s*[/,]\s*([\d.%]+))?\s*\)").unwrap();
    if let Some(caps) = css_hsl_re.captures(s) {
        let h = parse_hue(&caps[1]);
        let s = parse_component(&caps[2], 1.0);
        let l = parse_component(&caps[3], 1.0);
        return Some(hsl_to_rgb(h, s, l));
    }
    if let Some(rgb) = NAMED_COLORS.get(s.to_lowercase().as_str()) {
        return Some(*rgb);
    }
    None
}

fn scan_workspace() {
    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(_) => return,
    };

    let var_def_re = Regex::new(r"(--[\w-]+):\s*([^;!]+)(?:[! ]|;|\s|$)").unwrap();

    thread::spawn(move || {
        let walker = WalkBuilder::new(cwd).hidden(true).git_ignore(true).build();

        let mut new_vars = HashMap::new();
        for entry in walker.flatten() {
            if entry.file_type().is_some_and(|ft| ft.is_file()) {
                let path = entry.path();
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if matches!(ext, "css" | "scss" | "sass" | "less" | "postcss" | "styl") {
                    if let Ok(content) = fs::read_to_string(path) {
                        for caps in var_def_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            let val_str = caps[2].trim();
                            if let Some(rgb) = resolve_color_string(val_str) {
                                new_vars.insert(name, rgb);
                            }
                        }
                    }
                }
            }
        }

        if let Ok(mut vars) = CSS_VARIABLES.write() {
            *vars = new_vars;
        }
    });
}

fn scan_workspace_once() {
    if CSS_SCAN_STARTED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        scan_workspace();
    }
}

fn highlight_buffer(buf: &mut Buffer) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let ns_id = api::create_namespace("neocolor");
        let _ = buf.clear_namespace(ns_id, ..);

        let lines: Vec<String> = match buf.get_lines(.., false) {
            Ok(l) => l.map(|s: nvim_oxi::String| s.to_string()).collect(),
            Err(_) => return,
        };

        let ft_opts = OptionOpts::builder().buffer(buf.clone()).build();
        let ft: String = api::get_option_value("filetype", &ft_opts)
            .ok()
            .and_then(|o: Object| {
                if matches!(o.kind(), nvim_oxi::ObjectKind::String) {
                    let res: std::result::Result<nvim_oxi::String, nvim_oxi::conversion::Error> =
                        o.try_into();
                    res.ok().map(|s: nvim_oxi::String| s.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let hex_re = Regex::new(r"#[0-9a-fA-F]{3,8}").unwrap();
        let css_rgb_re = Regex::new(r"rgba?\(\s*([\d.%]+)\s*[, ]\s*([\d.%]+)\s*[, ]\s*([\d.%]+)(?:\s*[/,]\s*([\d.%]+))?\s*\)").unwrap();
        let css_hsl_re = Regex::new(r"hsla?\(\s*([\d.deggradturn]+)\s*[, ]\s*([\d.%]+)\s*[, ]\s*([\d.%]+)(?:\s*[/,]\s*([\d.%]+))?\s*\)").unwrap();
        let css_var_re = Regex::new(r"var\((--[\w-]+)\)").unwrap();
        let named_color_re = Regex::new(r"\b([a-zA-Z]{3,})\b").unwrap();

        let rust_rgb_re = Regex::new(r"rgb\(\s*0x([0-9a-fA-F]{6})\s*\)").unwrap();
        let rust_rgba_re = Regex::new(r"rgba\(\s*0x([0-9a-fA-F]{8})\s*\)").unwrap();
        let rust_hsla_re =
            Regex::new(r"hsla\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\)")
                .unwrap();

        let is_css_ft = match ft.as_str() {
            "css" | "scss" | "sass" | "less" | "postcss" | "stylus" | "html" | "javascript" | "typescript"
            | "javascriptreact" | "typescriptreact" | "vue" | "svelte" => true,
            _ => false,
        };

        if is_css_ft {
            scan_workspace_once();
        }

        for (i, line) in lines.iter().enumerate() {
            for mat in hex_re.find_iter(line) {
                let hex_str = mat.as_str().trim_start_matches('#');
                // Only color 3 and 4 character hex codes in CSS family files
                if (hex_str.len() == 3 || hex_str.len() == 4) && !is_css_ft {
                    continue;
                }
                if let Some((r, g, b)) = parse_hex(hex_str) {
                    apply_hl(buf, ns_id, i, mat.start(), mat.end(), r, g, b);
                }
            }

            if is_css_ft {
                for caps in css_rgb_re.captures_iter(line) {
                    if let (Some(m), Some(r_str), Some(g_str), Some(b_str)) =
                        (caps.get(0), caps.get(1), caps.get(2), caps.get(3))
                    {
                        let r = parse_component(r_str.as_str(), 255.0) as u8;
                        let g = parse_component(g_str.as_str(), 255.0) as u8;
                        let b = parse_component(b_str.as_str(), 255.0) as u8;
                        apply_hl(buf, ns_id, i, m.start(), m.end(), r, g, b);
                    }
                }
                for caps in css_hsl_re.captures_iter(line) {
                    if let (Some(m), Some(h_str), Some(s_str), Some(l_str)) =
                        (caps.get(0), caps.get(1), caps.get(2), caps.get(3))
                    {
                        let h = parse_hue(h_str.as_str());
                        let s = parse_component(s_str.as_str(), 1.0);
                        let l = parse_component(l_str.as_str(), 1.0);
                        let (r, g, b) = hsl_to_rgb(h, s, l);
                        apply_hl(buf, ns_id, i, m.start(), m.end(), r, g, b);
                    }
                }
                for caps in css_var_re.captures_iter(line) {
                    if let (Some(m), Some(var_name)) = (caps.get(0), caps.get(1)) {
                        if let Ok(vars) = CSS_VARIABLES.read() {
                            if let Some((r, g, b)) = vars.get(var_name.as_str()) {
                                apply_hl(buf, ns_id, i, m.start(), m.end(), *r, *g, *b);
                            }
                        }
                    }
                }
                for caps in named_color_re.captures_iter(line) {
                    if let (Some(m), Some(color_name)) = (caps.get(0), caps.get(1)) {
                        if let Some((r, g, b)) =
                            NAMED_COLORS.get(color_name.as_str().to_lowercase().as_str())
                        {
                            apply_hl(buf, ns_id, i, m.start(), m.end(), *r, *g, *b);
                        }
                    }
                }
            }

            if ft.starts_with("rust") {
                for caps in rust_rgb_re.captures_iter(line) {
                    if let (Some(m), Some(h)) = (caps.get(0), caps.get(1)) {
                        if let Some((r, g, b)) = parse_hex(h.as_str()) {
                            apply_hl(buf, ns_id, i, m.start(), m.end(), r, g, b);
                        }
                    }
                }
                for caps in rust_rgba_re.captures_iter(line) {
                    if let (Some(m), Some(h)) = (caps.get(0), caps.get(1)) {
                        let hex = h.as_str();
                        if hex.len() >= 6 {
                            if let Some((r, g, b)) = parse_hex(&hex[0..6]) {
                                apply_hl(buf, ns_id, i, m.start(), m.end(), r, g, b);
                            }
                        }
                    }
                }
                for caps in rust_hsla_re.captures_iter(line) {
                    if let (Some(m), Some(h), Some(s), Some(l)) =
                        (caps.get(0), caps.get(1), caps.get(2), caps.get(3))
                    {
                        if let (Ok(hv), Ok(sv), Ok(lv)) = (
                            h.as_str().parse::<f32>(),
                            s.as_str().parse::<f32>(),
                            l.as_str().parse::<f32>(),
                        ) {
                            let (r, g, b) = hsl_to_rgb(hv, sv, lv);
                            apply_hl(buf, ns_id, i, m.start(), m.end(), r, g, b);
                        }
                    }
                }
            }
        }
    }));
}

#[nvim_oxi::plugin]
fn neocolor_lib() -> Result<Dictionary> {
    // Rust-side setup takes NO arguments
    let setup: Function<(), ()> = Function::from_fn(|_| {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            if SETUP_DONE
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                return;
            }
            let callback = move |_args: AutocmdCallbackArgs| -> Result<bool> {
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    let mut buf = api::get_current_buf();
                    highlight_buffer(&mut buf);
                }));
                Ok(false)
            };
            let opts = CreateAutocmdOpts::builder().callback(callback).build();
            let _ = api::create_autocmd(vec!["BufEnter", "TextChanged", "TextChangedI"], &opts);
        }));
    });

    let update: Function<(), ()> = Function::from_fn(|_| {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut buf = api::get_current_buf();
            highlight_buffer(&mut buf);
        }));
    });

    Ok(Dictionary::from_iter([
        ("setup", Object::from(setup)),
        ("update", Object::from(update)),
    ]))
}
