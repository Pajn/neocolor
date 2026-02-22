use nvim_oxi::api::{self, opts::*, types::*, Buffer};
use nvim_oxi::{Dictionary, Function, Object, Result};
use regex::Regex;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 3 {
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
    let (r, g, b) = if h < 1.0 / 6.0 { (c, x, 0.0) }
    else if h < 2.0 / 6.0 { (x, c, 0.0) }
    else if h < 3.0 / 6.0 { (0.0, c, x) }
    else if h < 4.0 / 6.0 { (0.0, x, c) }
    else if h < 5.0 / 6.0 { (x, 0.0, c) }
    else { (c, 0.0, x) };
    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

fn apply_hl(buf: &mut Buffer, ns_id: u32, line: usize, start: usize, end: usize, r: u8, g: u8, b: u8) {
    let hl_name = format!("NeoColor_{:02x}{:02x}{:02x}", r, g, b);
    let bg = format!("#{:02x}{:02x}{:02x}", r, g, b);
    let brightness = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0;
    let fg = if brightness > 0.5 { "#000000" } else { "#ffffff" };

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
                    let res: std::result::Result<nvim_oxi::String, nvim_oxi::conversion::Error> = o.try_into();
                    res.ok().map(|s: nvim_oxi::String| s.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let hex_re = Regex::new(r"#[0-9a-fA-F]{3,8}").unwrap();
        let rust_rgb_re = Regex::new(r"rgb\(\s*0x([0-9a-fA-F]{6})\s*\)").unwrap();
        let rust_rgba_re = Regex::new(r"rgba\(\s*0x([0-9a-fA-F]{8})\s*\)").unwrap();
        let rust_hsla_re = Regex::new(r"hsla\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\)").unwrap();

        for (i, line) in lines.iter().enumerate() {
            for mat in hex_re.find_iter(line) {
                let hex = mat.as_str().trim_start_matches('#');
                if let Some((r, g, b)) = parse_hex(hex) {
                    apply_hl(buf, ns_id, i, mat.start(), mat.end(), r, g, b);
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
                    if let (Some(m), Some(h), Some(s), Some(l)) = (caps.get(0), caps.get(1), caps.get(2), caps.get(3)) {
                        if let (Ok(hv), Ok(sv), Ok(lv)) = (h.as_str().parse::<f32>(), s.as_str().parse::<f32>(), l.as_str().parse::<f32>()) {
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
