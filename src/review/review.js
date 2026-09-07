/* Runs directly from file://; no server, network requests, or browser packages. */
(() => {
  "use strict";
  const batch = window.REVIEW_BATCH;
  const el = id => document.getElementById(id);
  const groups = new Map(batch.groups.map(g => [g.id, g]));
  const initial = () => Object.fromEntries(batch.groups.map(g => [g.id, {
    selected: [...g.selected], reviewed: g.status === "reused", edited: false
  }]));
  const storageKey = `mistria-review-${batch.id}`;
  let decisions = initial();
  let current = null;
  let visible = [];
  let drawVersion = 0;
  let readyFrame = null;
  const message = text => { el("message").textContent = text; };
  function checkedProgress(value) {
    if (value.batch_id !== batch.id || !value.decisions ||
        Object.keys(value.decisions).length !== groups.size) throw new Error("Progress belongs to a different batch.");
    const result = {};
    for (const g of batch.groups) {
      const d = value.decisions[g.id];
      if (!d || typeof d.reviewed !== "boolean" || typeof d.edited !== "boolean" || !Array.isArray(d.selected) ||
          new Set(d.selected).size !== d.selected.length ||
          d.selected.some(i => !Number.isInteger(i) || i < 0 || i >= g.components.length))
        throw new Error("Invalid component selections in progress file.");
      result[g.id] = { selected: [...d.selected].sort((a,b) => a-b), reviewed: d.reviewed, edited: d.edited };
    }
    return result;
  }
  const progress = () => ({ batch_id: batch.id, decisions });
  try {
    const saved = localStorage.getItem(storageKey);
    if (saved) decisions = checkedProgress(JSON.parse(saved));
  } catch { message("Saved browser progress could not be loaded. Use a saved progress file if available."); }
  function persist() {
    try { localStorage.setItem(storageKey, JSON.stringify(progress())); }
    catch { message("Browser storage is unavailable. Use Save progress before closing."); }
  }
  function completeAssets() {
    return batch.assets.filter(a => a.frames.every(id => decisions[id].reviewed));
  }
  function suggest() {
    const votes = new Map(Object.entries(batch.reference_evidence).map(([signature, values]) => [signature, new Set(values)]));
    for (const g of batch.groups) {
      const d = decisions[g.id];
      const selections = d.reviewed ? [d.selected] : (d.edited ? [] : g.reference_selections ?? []);
      for (const selection of selections) {
        g.components.forEach((component,i) => {
          if (!votes.has(component.signature)) votes.set(component.signature, new Set());
          votes.get(component.signature).add(selection.includes(i));
        });
      }
    }
    for (const g of batch.groups) {
      const d = decisions[g.id];
      if (d.reviewed || d.edited || g.status === "conflict") continue;
      d.selected = g.components.flatMap((c,i) => {
        const vote = votes.get(c.signature);
        const selected = vote?.size === 1 && vote.has(true);
        return selected ? [i] : [];
      });
    }
  }
  function refresh() {
    const reviewed = batch.groups.filter(g => decisions[g.id].reviewed).length;
    const complete = completeAssets().length;
    el("summary").textContent = `${batch.summary.strips} strips · ${batch.summary.frames} frames · ${groups.size} unique frames · ${reviewed} reviewed · ${complete} strips ready to export`;
    el("export-profile").disabled = complete === 0;
    const query = el("search").value.toLowerCase();
    const filter = el("filter").value;
    visible = batch.groups.filter(g => (filter === "all" || decisions[g.id].reviewed === (filter === "reviewed")) &&
      g.occurrences.some(o => o.asset.toLowerCase().includes(query)));
    if (!visible.some(g => g.id === current)) current = visible[0]?.id ?? null;
    const list = el("frames");
    list.replaceChildren();
    for (const g of visible) {
      const button = document.createElement("button");
      button.dataset.group = g.id;
      button.setAttribute("aria-current", String(g.id === current));
      const img = document.createElement("img");
      img.src = g.image; img.alt = ""; img.loading = "lazy";
      const title = document.createElement("span");
      title.textContent = `${g.occurrences[0].asset.split("/").pop()} · frame ${g.occurrences[0].frame} · ${g.occurrences.length} uses`;
      button.append(img, title);
      button.onclick = () => { current = g.id; refresh(); };
      list.append(button);
    }
    draw();
  }
  function draw() {
    const version = ++drawVersion;
    readyFrame = null;
    el("approve").disabled = true;
    el("clear").disabled = true;
    document.querySelector(".previews").style.opacity = ".4";
    el("editor").hidden = current === null;
    if (current === null) return;
    const g = groups.get(current), d = decisions[current];
    const occurrence = g.occurrences[0];
    el("frame-title").textContent = `${occurrence.asset.split("/").pop()} — frame ${occurrence.frame}`;
    const labels = {reused:"Mask reused from identical approved artwork",suggested:"Suggested regions from matching approved components",conflict:"Conflicting reference masks — select the intended regions",unreviewed:"No approved match — select the skin regions"};
    const origin = !d.edited && !d.reviewed && d.selected.length ? "Suggested regions from matching approved components" : labels[g.status];
    el("status").textContent = `${d.reviewed ? "Reviewed" : "Needs review"}. ${origin}. ${d.selected.length}/${g.components.length} regions selected.`;
    const index = visible.findIndex(x => x.id === current);
    el("previous").disabled = index === 0;
    el("next").disabled = index === visible.length-1;
    el("occurrences").textContent = g.occurrences.map(o => `${o.asset} [frame ${o.frame}]`).join("\n") +
      (g.references.length ? "\n\nApproved references:\n" + g.references.map(o => `${o.asset} [frame ${o.frame}]`).join("\n") : "");
    const img = new Image();
    img.onerror = () => { if (version === drawVersion) message("Could not load the frame image. Keep the gallery files together."); };
    img.onload = () => {
      if (version !== drawVersion) return;
      const [left,top,width,height] = g.bounds;
      const zoom = el("zoom").value === "auto" ? (Math.max(...g.size) <= 96 ? 6 : 2) : Number(el("zoom").value);
      for (const name of ["original","blue","mask"]) {
        const canvas = el(name);
        canvas.width = width; canvas.height = height;
        canvas.style.width = `${width * zoom}px`;
        canvas.style.height = `${height * zoom}px`;
        const ctx = canvas.getContext("2d");
        ctx.imageSmoothingEnabled = false;
        ctx.globalAlpha = name === "mask" ? .25 : 1;
        ctx.drawImage(img,left,top,width,height,0,0,width,height);
        ctx.globalAlpha = 1;
        if (name === "original") continue;
        g.components.forEach((component,i) => {
          const selected = d.selected.includes(i);
          if (name === "blue" && !selected) return;
          for (const [start,length,shade] of component.runs) {
            const target = batch.preview_colors[shade];
            ctx.fillStyle = name === "mask" ? (selected ? "#ff55cc" : "#efb04f") : `rgb(${target.slice(0,3).join(",")})`;
            ctx.fillRect(start % g.size[0]-left, Math.floor(start/g.size[0])-top, length, 1);
          }
        });
      }
      readyFrame = g.id;
      el("approve").disabled = d.reviewed;
      el("clear").disabled = false;
      document.querySelector(".previews").style.opacity = "1";
    };
    img.src = g.image;
  }
  function edit(selection) {
    decisions[current] = {selected:selection.sort((a,b)=>a-b),reviewed:false,edited:true};
    suggest(); persist(); refresh();
  }
  for (const name of ["original","blue","mask"]) el(name).onclick = event => {
    if (current === null || readyFrame !== current) return;
    const g = groups.get(current), canvas = el(name), box = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX-box.left)*canvas.width/box.width)+g.bounds[0];
    const y = Math.floor((event.clientY-box.top)*canvas.height/box.height)+g.bounds[1];
    if (x<0 || y<0 || x>=g.size[0] || y>=g.size[1]) return;
    const pixel = y*g.size[0]+x;
    const component = g.components.findIndex(c => c.runs.some(([start,length])=>pixel>=start && pixel<start+length));
    if (component < 0) { message("That pixel is outside the configured source colors."); return; }
    const selected = new Set(decisions[current].selected);
    if (selected.has(component)) selected.delete(component); else selected.add(component);
    edit([...selected]);
  };
  el("clear").onclick = () => { if(current !== null && readyFrame === current) edit([]); };
  el("approve").onclick = () => {
    if(current === null || readyFrame !== current) return;
    decisions[current].reviewed = true; suggest(); persist(); refresh();
  };
  for (const [name,delta] of [["previous",-1],["next",1]]) el(name).onclick = () => {
    const index = visible.findIndex(g=>g.id===current)+delta;
    if (visible[index]) { current=visible[index].id; refresh(); }
  };
  el("filter").onchange = refresh; el("search").oninput = refresh; el("zoom").onchange = draw;
  function download(name,value) {
    const blob = new Blob([JSON.stringify(value,null,2)+"\n"],{type:"application/json"});
    const url = URL.createObjectURL(blob), anchor=document.createElement("a");
    anchor.href=url; anchor.download=name; anchor.click();
    setTimeout(()=>URL.revokeObjectURL(url),1000);
  }
  el("save-progress").onclick = () => download("review-progress.json",progress());
  el("load-progress").onchange = async event => {
    const file=event.target.files[0]; if(!file) return;
    try { decisions=checkedProgress(JSON.parse(await file.text())); persist(); refresh(); message("Progress loaded."); }
    catch(error) { message(error.message); }
    event.target.value="";
  };
  el("export-profile").onclick = () => {
    const assets=completeAssets(); if(!assets.length) return;
    const regions=assets.map(a=>({asset:a.asset,source_sha256:a.source_sha256,size:a.size,
      seeds:a.frames.flatMap((id,frame)=>decisions[id].selected.map(i=>{
        const [x,y]=groups.get(id).components[i].seed; return [x+frame*a.frame_size[0],y];
      }))}));
    download("reviewed-profile.json",{source_colors:batch.source_colors,color_groups:batch.color_groups ?? [],regions});
    message(`Exported ${assets.length} reviewed strips; ${batch.assets.length-assets.length} incomplete strips excluded.`);
  };
  el("title").textContent=batch.label;
  document.title=`${batch.label} — palette review`;
  refresh();
})();
