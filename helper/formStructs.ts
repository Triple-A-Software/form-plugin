const commonProps = [
	{
		id: "label",
		type: "text",
		label: { de: "Label", en: "Label" },
	},
	{
		id: "required",
		type: "checkbox",
		label: { de: "Pflichtfeld", en: "Required" },
		defaultValue: false,
	},
	{
		id: "class",
		type: "text",
		label: { de: "CSS Klassen", en: "CSS Classes" },
	},
];

export const formBuilderItems = [
	{
		id: "text",
		label: {
			de: "Text",
			en: "Text",
		},
		icon: "i-tabler-letter-case",
		props: [
			...commonProps,
			{
				id: "placeholder",
				type: "text",
				label: { de: "Platzhalter", en: "Placeholder" },
			},
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
	},
	{
		id: "checkbox",
		label: {
			de: "Kontrollkästchen",
			en: "Checkbox",
		},
		icon: "i-tabler-checkbox",
		props: [...commonProps],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "p-2",
		},
	},
	{
		id: "textarea",
		label: {
			de: "Textfeld",
			en: "Textarea",
		},
		props: [...commonProps],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
	},
	{
		id: "button",
		label: {
			de: "Button",
			en: "Button",
		},
		icon: "i-tabler-square-arrow-down",
		props: [
			...commonProps,
			{
				id: "type",
				type: "buttongroup",
				label: { de: "Buttontyp", en: "Button type" },
				options: [
					{ label: "Submit", value: "submit" },
					{ label: "Reset", value: "reset" },
					{ label: "Button", value: "button" },
				],
			},
			{
				id: "onclick",
				type: "code",
				language: "js",
				label: { de: "onClick Funktion", en: "onClick Function" },
			},
		],
		data: { label: "Submit", class: "", type: "submit", onClick: "" },
	},
];
