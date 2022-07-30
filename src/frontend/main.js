document.addEventListener('contextmenu', event => event.preventDefault())

function getLang() {
	window.__TAURI__.invoke('get_ui_btns').then((res) => {
		document.getElementById('insert_num').placeholder = res[0].replace(/:/g, '')
		document.getElementById('convert_btn').textContent = res[1]
		document.getElementById('clear_btn').textContent = res[2]
		document.getElementById('copy_btn').textContent = res[3]
	})
}

function convertInput() {
	window.__TAURI__.invoke('convert', { input: document.getElementById('insert_num').value }).then((res) => {
		document.getElementById('output').textContent = res
	})
}

function copyOutput() {
	window.__TAURI__.clipboard.writeText(document.getElementById('output').textContent)
}

function clearIO() {
	document.getElementById('insert_num').value = ''
	document.getElementById('output').textContent = ''
}