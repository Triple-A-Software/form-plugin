export function textInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <input type="text"
				   name="${data.label + index}"
				   required="${data.required}" 
				   placeholder="${data.placeholder}" />
        </div>`;
}

export function checkboxInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <input type="checkbox"
				   name="${data.label + index}"
				   required="${data.required}" 
				   />
        </div>`;
}

export function textAreaInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <textarea 
				name="${data.label + index}"
				required="${data.required}" 
				placeholder="${data.placeholder}" >
			</textarea>
        </div>`;
}

export function selectInput(data, index) {
	const options = data.options.map(
		(item, index) =>
			`<option value=${item.value} key=${index}>${item.name}</option>`,
	);

	return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <select 
				   name="${data.label + index}"
				   required="${data.required}" 
				   placeholder="${data.placeholder}" >
					${options}
				   </select>
        </div>`;
}
