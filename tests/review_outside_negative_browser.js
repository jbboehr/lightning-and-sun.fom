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
  let blob;
  URL.createObjectURL=value=>{blob=value;return "blob:test";};
  URL.revokeObjectURL=()=>{};
  document.addEventListener("click",event=>{if(event.target.tagName==="A")event.preventDefault();});
  async function progress() {
    blob=null;el("save-progress").click();assert(blob,"Progress was not downloaded");return JSON.parse(await blob.text());
  }
  try {
    await wait();
    const first=window.REVIEW_BATCH.groups.find(group=>group.occurrences.some(o=>o.asset.endsWith("/first.png")));
    const second=window.REVIEW_BATCH.groups.find(group=>group.occurrences.some(o=>o.asset.endsWith("/second.png")));
    assert(first && second && first.id!==second.id,"Selected frames were not separate review groups");
    assert(JSON.stringify(first.selected)==="[]" && JSON.stringify(second.selected)==="[]","Outside negative evidence did not suppress the initial suggestions");
    document.querySelector(`[data-group="${first.id}"]`).click();await wait();
    const canvas=el("original"),box=canvas.getBoundingClientRect();
    canvas.dispatchEvent(new MouseEvent("click",{clientX:box.left+.5/3*box.width,clientY:box.top+.5*box.height,bubbles:true}));
    await wait();el("approve").click();
    const saved=await progress();
    assert(JSON.stringify(saved.decisions[second.id].selected)==="[]","Approved negative evidence was lost during live suggestion voting");
    const conflict=window.REVIEW_BATCH.groups.find(group=>group.status==="conflict");
    if(conflict) {
      document.querySelector(`[data-group="${conflict.id}"]`).click();await wait();
      const canvas=el("original"),box=canvas.getBoundingClientRect();
      canvas.dispatchEvent(new MouseEvent("click",{clientX:box.left+.5/3*box.width,clientY:box.top+.5*box.height,bubbles:true}));
      await wait();el("approve").click();
      const resolved=await progress();
      assert(JSON.stringify(resolved.decisions[second.id].selected)==="[0]","Resolving the reference conflict did not replace its old negative evidence");
      assert(!resolved.decisions[second.id].reviewed,"Resolving a conflict silently approved a suggestion");
    }
    document.body.dataset.testResult="passed";
  } catch(error) { document.body.dataset.testResult="failed";el("message").textContent=error.stack; }
});
