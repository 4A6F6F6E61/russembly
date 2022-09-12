function out(s) {
    console.log(s)
    const russemblyOutputTextarea = document.getElementById(
        "russemblyOutputTextarea"
    )
    russemblyOutputTextarea.value += `${s}\n`
    russemblyOutputTextarea.scrollTo({
        top: russemblyOutputTextarea.scrollHeight,
        behavior: "smooth",
    })
}

export { out }
