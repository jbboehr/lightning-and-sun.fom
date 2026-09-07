window.addEventListener("load", async () => {
  const el=id=>document.getElementById(id);
  const assert=(value,message)=>{if(!value)throw new Error(message);};
  const wait=()=>new Promise(resolve=>{
    const previews=document.querySelector(".previews");
    if(el("editor").hidden || previews.style.opacity==="1") {resolve();return;}
    const observer=new MutationObserver(()=>{
      if(el("editor").hidden || previews.style.opacity==="1") {observer.disconnect();resolve();}
    });
    observer.observe(previews,{attributes:true,attributeFilter:["style"]});
    observer.observe(el("editor"),{attributes:true,attributeFilter:["hidden"]});
  });
  // Observe the real async file-input handler instead of sleeping while File.text runs.
  let loading;
  const loadHandler=el("load-progress").onchange;
  el("load-progress").onchange=event=>{loading=loadHandler(event);};
  let blob;
  URL.createObjectURL=value=>{blob=value;return "blob:test";};
  URL.revokeObjectURL=()=>{};
  document.addEventListener("click",event=>{if(event.target.tagName==="A")event.preventDefault();});
  async function downloaded(id) {blob=null;el(id).click();assert(blob,`No download from ${id}`);return JSON.parse(await blob.text());}
  async function load(value) {
    const transfer=new DataTransfer();
    transfer.items.add(new File([JSON.stringify(value)],"progress.json",{type:"application/json"}));
    el("load-progress").files=transfer.files;
    el("load-progress").dispatchEvent(new Event("change"));await loading;await wait();
  }
  try {
    await wait();
    assert(el("title").textContent==="Test <character>","Label was not preserved as text");
    const partial=await downloaded("export-profile");
    assert(JSON.stringify(partial.color_groups)==='[["#C86432"]]',"Export lost color group boundaries");
    assert(partial.regions.length===2,"Unreviewed strip exported");
    assert(el("frame-title").textContent.includes("new.png"),"Pending filter did not select new frame");
    // A redraw has not loaded its image yet. Do not interpret clicks on the old canvas.
    const stale=el("mask"),oldBox=stale.getBoundingClientRect();
    el("zoom").value="4";el("zoom").dispatchEvent(new Event("change"));
    stale.dispatchEvent(new MouseEvent("click",{clientX:oldBox.left+3.5/5*oldBox.width,clientY:oldBox.top+.5/3*oldBox.height,bubbles:true}));
    const loading=await downloaded("save-progress");
    const pendingFrame=window.REVIEW_BATCH.groups.find(g=>g.occurrences.some(o=>o.asset.endsWith("/new.png")));
    assert(JSON.stringify(loading.decisions[pendingFrame.id].selected)==="[0]","Accepted edits before the frame image loaded");
    await wait();
    const canvas=el("mask"),rect=canvas.getBoundingClientRect();
    canvas.dispatchEvent(new MouseEvent("click",{clientX:rect.left+3.5/5*rect.width,clientY:rect.top+.5/3*rect.height,bubbles:true}));
    await wait();
    assert(el("status").textContent.includes("2/3 regions selected"),"Click did not select the disconnected hand");
    const before=await downloaded("save-progress");
    el("approve").click();
    const full=await downloaded("export-profile");
    assert(full.regions.length===3,"Approved strip missing");
    const added=full.regions.find(r=>r.asset.endsWith("/new.png"));
    assert(JSON.stringify(added.seeds)==="[[1,0],[3,0]]","Incorrect exported region seeds");
    await load(before);
    assert((await downloaded("export-profile")).regions.length===2,"Loading progress did not restore pending approval");
    const wrong={...before,batch_id:"different"};
    await load(wrong);
    assert(el("message").textContent.includes("different batch"),"Foreign progress accepted");
    assert((await downloaded("export-profile")).regions.length===2,"Foreign progress mutated decisions");
    el("approve").click();
    el("filter").value="reviewed";el("filter").dispatchEvent(new Event("change"));
    const g=window.REVIEW_BATCH.groups.find(g=>g.occurrences.some(o=>o.asset.endsWith("/duplicate.png")));
    document.querySelector(`[data-group="${g.id}"]`).click();await wait();
    el("clear").click();
    // Changing a shared frame invalidates both the source strip and its duplicate.
    assert((await downloaded("export-profile")).regions.length===1,"Editing a reviewed shared frame retained approval");
    // Start a new character with no approved masks. One reviewed face should
    // suggest matching components without approving its other expressions.
    await load({batch_id:window.REVIEW_BATCH.id,decisions:Object.fromEntries(window.REVIEW_BATCH.groups.map(g=>[g.id,{selected:[],reviewed:false,edited:false}]))});
    el("filter").value="all";el("filter").dispatchEvent(new Event("change"));
    document.querySelector(`[data-group="${g.id}"]`).click();await wait();
    const original=el("original"),box=original.getBoundingClientRect();
    original.dispatchEvent(new MouseEvent("click",{clientX:box.left+1.5/5*box.width,clientY:box.top+.5/3*box.height,bubbles:true}));
    await wait();
    el("approve").click();
    const suggested=await downloaded("save-progress");
    const newFrame=window.REVIEW_BATCH.groups.find(g=>g.occurrences.some(o=>o.asset.endsWith("/new.png")));
    assert(JSON.stringify(suggested.decisions[newFrame.id].selected)==="[0]","Newly approved regions were not suggested");
    assert(!suggested.decisions[newFrame.id].reviewed,"Suggestion was silently approved");
    // A later negative review of the same component signature must withdraw the
    // suggestion, while subsequent recomputation must preserve manual edits.
    const negative=window.REVIEW_BATCH.groups.find(group=>group.id!==g.id && group.id!==newFrame.id);
    document.querySelector(`[data-group="${negative.id}"]`).click();await wait();
    const negativeCanvas=el("original"),negativeBox=negativeCanvas.getBoundingClientRect();
    negativeCanvas.dispatchEvent(new MouseEvent("click",{clientX:negativeBox.left+1.5/5*negativeBox.width,clientY:negativeBox.top+.5/3*negativeBox.height,bubbles:true}));
    await wait();el("approve").click();
    const conflicted=await downloaded("save-progress");
    assert(JSON.stringify(conflicted.decisions[newFrame.id].selected)==="[]","Conflicting positive and negative evidence retained a positive suggestion");
    document.querySelector(`[data-group="${newFrame.id}"]`).click();await wait();
    const manualCanvas=el("original"),manualBox=manualCanvas.getBoundingClientRect();
    manualCanvas.dispatchEvent(new MouseEvent("click",{clientX:manualBox.left+3.5/5*manualBox.width,clientY:manualBox.top+.5/3*manualBox.height,bubbles:true}));
    await wait();
    document.querySelector(`[data-group="${negative.id}"]`).click();await wait();
    const revisedCanvas=el("original"),revisedBox=revisedCanvas.getBoundingClientRect();
    revisedCanvas.dispatchEvent(new MouseEvent("click",{clientX:revisedBox.left+1.5/5*revisedBox.width,clientY:revisedBox.top+.5/3*revisedBox.height,bubbles:true}));
    await wait();el("approve").click();
    const preserved=await downloaded("save-progress");
    assert(JSON.stringify(preserved.decisions[newFrame.id].selected)==="[1]","Suggestion recomputation replaced a manual edit");
    // Withdrawing every approved reference also withdraws its untouched suggestions.
    await load({batch_id:window.REVIEW_BATCH.id,decisions:Object.fromEntries(window.REVIEW_BATCH.groups.map(group=>[group.id,{selected:[...group.selected],reviewed:group.status==="reused",edited:false}]))});
    for (const reference of window.REVIEW_BATCH.groups.filter(group=>group.status==="reused")) {
      document.querySelector(`[data-group="${reference.id}"]`).click();await wait();
      el("clear").click();await wait();
    }
    const withdrawn=await downloaded("save-progress");
    assert(JSON.stringify(withdrawn.decisions[newFrame.id].selected)==="[]","Withdrawing all reference approvals retained a stale suggestion");
    const pre=document.createElement("pre");pre.id="exported-profile";pre.textContent=JSON.stringify(full);document.body.append(pre);
    document.body.dataset.testResult="passed";
  } catch(error) { document.body.dataset.testResult="failed";el("message").textContent=error.stack; }
});
