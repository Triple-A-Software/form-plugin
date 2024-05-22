function textInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="text_${index}">
			${data.label}
			</label>
            <input type="text"
				   name="text_${index}"
				   required="${data.required}" 
				   placeholder="${data.placeholder}" />
        </div>`;
}

function checkboxInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="checkbox_${index}">
			${data.label}
			</label>
            <input type="checkbox"
				   name="checkbox_${index}"
				   required="${data.required}" 
				   />
        </div>`;
}

function textAreaInput(data, index) {
	return `
        <div class="${data.class}">
            <label htmlFor="textarea_${index}">
			${data.label}
			</label>
            <textarea 
				name="textarea_${index}"
				required="${data.required}" 
				placeholder="${data.placeholder}" >
			</textarea>
        </div>`;
}

function selectInput(data, index) {
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

function inputButton(data) {
	return `
        <button type="${data.type}" onclick="${data.onclick}" class="${data.class}">
			${data.label}
        </button>`;
}

export function buildForm(data) {
	return data
		.map((input, index) => {
			const { id, data } = input;
			switch (id) {
				case "text":
					return textInput(data, index);
				case "textarea":
					return textAreaInput(data, index);
				case "checkbox":
					return checkboxInput(data, index);
				case "button":
					return inputButton(data);
				//TODO: add new Elements for the Builder here
				default:
					return "";
			}
		})
		.join("");
}
