

window.onload = ()=>{
    const pItem = window.document.getElementById("p_time");
    const now = new Date();
    pItem.innerText = `${now.toString()}`
};