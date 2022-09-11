
function js_output(s) {
  if (!localStorage.russembly_output) {
    localStorage.russembly_output = ""
  }
  localStorage.russembly_output += `${s}`
}
