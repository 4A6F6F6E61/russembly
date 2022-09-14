/*function out(s) {
    console.log(s)
    const russemblyOutputTextarea = document.getElementById(
        "russemblyOutputTextarea"
    )
    russemblyOutputTextarea.value += `${s}\n`
    russemblyOutputTextarea.scrollTo({
        top: russemblyOutputTextarea.scrollHeight,
        behavior: "smooth",
    })
}*/

function out(s) {
    /*if (!localStorage.wasm_output) {
        localStorage.setItem("wasm_output", "")
    }
    localStorage.wasm_output += `${s}\n`*/
    dispatchEvent(new Event("wasm_out_changed"))
}

function clear_ls() {
    localStorage.setItem("wasm_output", "")
}

export { out, clear_ls }
