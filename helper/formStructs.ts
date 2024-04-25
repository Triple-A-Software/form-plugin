import { checkboxInput } from "./utils";

export const formBuilderItems = [
	{
		id: "text",
		label: {
			de: "text",
			en: "text",
		},
		icon: "i-tabler-letter-case",
		props: [
			{
				id: "label",
				type: "text",
				label: { de: "Label", en: "Label" },
			},
			{
				id: "placeholder",
				type: "text",
				label: { de: "Platzhalter", en: "Placeholder" },
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
		props: [
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
		],
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
		props: [
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
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
	},
	{
		id: "select",
		label: {
			de: "Auswahl",
			en: "Select",
		},
		icon: "i-tabler-select",
		props: [
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
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
			options: [
				/*TODO: How to Add Optiont here?*/
			],
		},
	},
];
