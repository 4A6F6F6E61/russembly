function out(s) {
    console.log(s)
    const russemblyOutputTextarea = document.getElementById(
        "russemblyOutputTextarea"
    )
    russemblyOutputTextarea.value += `${s}\n`
}

export { out }
