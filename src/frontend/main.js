
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

async function toggleDark(doSleep) {
	const sleep = ms => new Promise(r => setTimeout(r, ms))
	const delay = doSleep ? 300 : 0
	document.querySelector('.theme-toggle').classList.toggle('dark')
	document.querySelector('body').classList.toggle('dark')
	await sleep(delay)
	document.querySelector('#insert_num').classList.toggle('dark')
	await sleep(delay)
	document.querySelector('#convert_btn').classList.toggle('dark')
	await sleep(delay)
	document.querySelector('#copy_btn').classList.toggle('dark')
	await sleep(delay)
	document.querySelector('#clear_btn').classList.toggle('dark')
	await sleep(delay)
	document.querySelector('#output-container').classList.toggle('dark')
	document.querySelector('#output').classList.toggle('dark')
}
