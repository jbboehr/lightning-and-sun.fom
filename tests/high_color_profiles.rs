use image::{DynamicImage, Rgba, RgbaImage};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use zip::{ZipWriter, write::SimpleFileOptions};

const ASSET: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral.png";

fn fixture(count: u32) -> (tempfile::TempDir, PathBuf, PathBuf, PathBuf, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let image = RgbaImage::from_fn(count.max(1) + 2, 1, |x, _| {
        if x < count {
            Rgba([x as u8 + 1, 20, 30, 255])
        } else if count > 0 && x == count + 1 {
            Rgba([count as u8, 20, 30, 255])
        } else {
            Rgba([0, 0, 0, 0])
        }
    });
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image.clone())
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let bytes = bytes.into_inner();
    let metadata = format!(
        "[meta_properties]\nid='source'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[{},1]\natlas='PortraitsSpring'\n",
        image.width()
    );
    let path = original.join(ASSET);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, &bytes).unwrap();
    fs::write(path.with_extension("meta.toml"), &metadata).unwrap();
    let source: Vec<_> = (0..count)
        .rev()
        .map(|i| format!("#{:02X}141E", i + 1))
        .collect();
    let targets: Vec<_> = (0..count)
        .rev()
        .map(|i| format!("#{:02X}5A64", i + 128))
        .collect();
    let groups: Vec<_> = source.iter().map(|c| vec![c]).collect();
    let seeds: Vec<_> = (0..count)
        .filter(|i| *i != count / 2)
        .map(|i| [i, 0])
        .collect();
    let profile = json!({"source_colors":source,"color_groups":groups,"regions":[{"asset":ASSET,"source_sha256":format!("{:x}",Sha256::digest(&bytes)),"size":[image.width(),1],"seeds":seeds}]});
    fs::write(
        temp.path().join("profile.json"),
        serde_json::to_vec(&profile).unwrap(),
    )
    .unwrap();
    let set = temp.path().join("presets.json");
    fs::write(&set,serde_json::to_vec(&json!({"profile":"profile.json","presets":[{"id":"blue","label":"Debug Blue","colors":targets}]})).unwrap()).unwrap();
    let archive = temp.path().join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for (name, data) in [
        (ASSET.to_owned(), bytes),
        (ASSET.replace(".png", ".meta.toml"), metadata.into_bytes()),
    ] {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&data).unwrap();
    }
    zip.finish().unwrap();
    let config = temp.path().join("review.json");
    fs::write(&config,serde_json::to_vec(&json!({"label":"Many real shades","prefixes":["assets/animations/NPCs/Adeline/Portraits/Spring/"],"source_colors":source,"preview_colors":targets,"reference_profile":"profile.json"})).unwrap()).unwrap();
    (temp, original, set, archive, config)
}
fn run(args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn presets_recolor_real_shades_through_index_63_and_preserve_unselected_matches() {
    for count in [32, 33, 64] {
        let (temp, original, set, _, _) = fixture(count);
        let output = temp.path().join("bundle");
        let result = run(&[
            Path::new("build-presets"),
            Path::new("--original"),
            &original,
            Path::new("--presets"),
            &set,
            Path::new("--output"),
            &output,
        ]);
        assert!(
            result.status.success(),
            "{count}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let actual = image::open(output.join("variants/blue").join(ASSET))
            .unwrap()
            .to_rgba8();
        for x in 0..count {
            let expected = if x == count / 2 {
                [x as u8 + 1, 20, 30, 255]
            } else {
                [x as u8 + 128, 90, 100, 255]
            };
            assert_eq!(
                actual.get_pixel(x, 0).0,
                expected,
                "source shade {x} of{count}"
            );
        }
        assert_eq!(actual.get_pixel(count + 1, 0).0, [count as u8, 20, 30, 255]);
        assert_eq!(actual.get_pixel(count, 0).0, [0, 0, 0, 0]);
    }
}
#[test]
fn review_preserves_shuffled_color_mappings_and_high_component_indices() {
    let (temp, _, _, archive, config) = fixture(64);
    let output = temp.path().join("review");
    let result = run(&[
        Path::new("review-batch"),
        Path::new("--archive"),
        &archive,
        Path::new("--config"),
        &config,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(report["source_colors"][0], "#40141E");
    assert_eq!(report["colors"][63], json!([64, 20, 30, 255]));
    assert_eq!(report["preview_colors"][63], json!([191, 90, 100, 255]));
    let group = &report["groups"][0];
    assert_eq!(group["components"].as_array().unwrap().len(), 65);
    assert_eq!(group["components"][63]["runs"], json!([[63, 1, 63]]));
    assert_eq!(group["components"][64]["runs"], json!([[65, 1, 63]]));
    let selected = group["selected"].as_array().unwrap();
    assert!(selected.contains(&json!(63)));
    assert!(!selected.contains(&json!(32)));
    assert!(!selected.contains(&json!(64)));
}
#[test]
fn empty_and_over_limit_color_lists_fail_before_publishing_outputs() {
    for count in [0, 65] {
        let (temp, original, set, archive, config) = fixture(count);
        for (name, args) in [
            (
                "presets",
                vec![
                    Path::new("build-presets"),
                    Path::new("--original"),
                    &original,
                    Path::new("--presets"),
                    &set,
                ],
            ),
            (
                "review",
                vec![
                    Path::new("review-batch"),
                    Path::new("--archive"),
                    &archive,
                    Path::new("--config"),
                    &config,
                ],
            ),
        ] {
            let output = temp.path().join(name);
            let mut args = args;
            args.extend([Path::new("--output"), &output]);
            let result = run(&args);
            assert!(!result.status.success(), "{count} accepted by{name}");
            assert!(!output.exists());
        }
    }
}

#[test]
#[ignore = "requires FOM_REVIEW_BROWSER pointing to a Chromium executable"]
fn browser_renders_exports_and_restores_components_above_index_31() {
    let (temp, original, set, archive, config) = fixture(64);
    let output = temp.path().join("review");
    let result = run(&[
        Path::new("review-batch"),
        Path::new("--archive"),
        &archive,
        Path::new("--config"),
        &config,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let script = r#"
addEventListener('load',async()=>{
 const el=id=>document.getElementById(id),assert=(ok,message)=>{if(!ok)throw Error(message)};
 const wait=()=>new Promise(resolve=>{const p=document.querySelector('.previews');if(el('editor').hidden||p.style.opacity==='1'){resolve();return}const o=new MutationObserver(()=>{if(el('editor').hidden||p.style.opacity==='1'){o.disconnect();resolve()}});o.observe(p,{attributes:true,attributeFilter:['style']});o.observe(el('editor'),{attributes:true,attributeFilter:['hidden']})});
 let blob,loading;const handler=el('load-progress').onchange;el('load-progress').onchange=e=>{loading=handler(e)};
 URL.createObjectURL=value=>{blob=value;return 'blob:test'};URL.revokeObjectURL=()=>{};document.addEventListener('click',e=>{if(e.target.tagName==='A')e.preventDefault()});
 const download=async id=>{blob=null;el(id).click();assert(blob,'Missing download');return JSON.parse(await blob.text())};
 try{
  el('filter').value='all';el('filter').dispatchEvent(new Event('change'));await wait();
  assert([...el('blue').getContext('2d').getImageData(63,0,1,1).data].join(',')==='191,90,100,255','Wrong shade63 preview');
  assert([...el('blue').getContext('2d').getImageData(65,0,1,1).data].join(',')==='64,20,30,255','Unselected duplicate recolored');
  const canvas=el('mask'),box=canvas.getBoundingClientRect();canvas.dispatchEvent(new MouseEvent('click',{clientX:box.left+65.5/66*box.width,clientY:box.top+.5*box.height,bubbles:true}));await wait();
  const saved=await download('save-progress'),group=window.REVIEW_BATCH.groups[0];assert(saved.decisions[group.id].selected.includes(64),'High component not selected');assert(!saved.decisions[group.id].selected.includes(32),'Excluded component changed');
  el('clear').click();await wait();const transfer=new DataTransfer();transfer.items.add(new File([JSON.stringify(saved)],'progress.json',{type:'application/json'}));el('load-progress').files=transfer.files;el('load-progress').dispatchEvent(new Event('change'));await loading;await wait();
  const restored=await download('save-progress');assert(JSON.stringify(saved.decisions)===JSON.stringify(restored.decisions),'Progress roundtrip changed selections');
  el('approve').click();await wait();const exported=await download('export-profile');assert(exported.source_colors.length===64&&exported.color_groups.length===64,'Export truncated colors');assert(exported.regions[0].seeds.some(s=>s[0]===65),'Export lost high component');
  const pre=document.createElement('pre');pre.id='exported-profile';pre.textContent=JSON.stringify(exported);document.body.append(pre);document.body.dataset.testResult='passed';
 }catch(error){document.body.dataset.testResult='failed';el('message').textContent=error.stack}
});
"#;
    let page = output.join("index.html");
    let html = fs::read_to_string(&page).unwrap();
    fs::write(
        &page,
        html.replace("</html>", &format!("<script>{script}</script></html>")),
    )
    .unwrap();
    let browser = std::env::var_os("FOM_REVIEW_BROWSER").expect("Set FOM_REVIEW_BROWSER");
    let result = Command::new(browser)
        .args([
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage",
            "--allow-file-access-from-files",
            "--no-first-run",
            "--dump-dom",
            "--virtual-time-budget=5000",
        ])
        .arg(format!(
            "--user-data-dir={}",
            temp.path().join("browser").display()
        ))
        .arg(format!("file://{}", page.display()))
        .output()
        .unwrap();
    let dom = String::from_utf8(result.stdout).unwrap();
    assert!(
        result.status.success() && dom.contains("data-test-result=\"passed\""),
        "{dom}\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let serialized = dom
        .split("<pre id=\"exported-profile\">")
        .nth(1)
        .unwrap()
        .split("</pre>")
        .next()
        .unwrap();
    let exported: Value = serde_json::from_str(serialized).unwrap();
    fs::write(
        temp.path().join("profile.json"),
        serde_json::to_vec(&exported).unwrap(),
    )
    .unwrap();
    let bundle = temp.path().join("browser-bundle");
    let result = run(&[
        Path::new("build-presets"),
        Path::new("--original"),
        &original,
        Path::new("--presets"),
        &set,
        Path::new("--output"),
        &bundle,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let actual = image::open(bundle.join("variants/blue").join(ASSET))
        .unwrap()
        .to_rgba8();
    assert_eq!(actual.get_pixel(65, 0).0, [191, 90, 100, 255]);
    assert_eq!(actual.get_pixel(32, 0).0, [33, 20, 30, 255]);
}
