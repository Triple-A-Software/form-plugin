import type en from "./en";

export default {
	action: {
		create: {
			field: "Feld erstellen",
			option: "Option erstellen",
		},
	},
	modal: {
		edit: {
			field: {
				title: "Feld bearbeiten: {type}",
			},
		},
	},
	field: {
		label: {
			formbuilder: "Form-builder",
			type: "Typ",
			max: "Max",
			maxLength: "Maximale Länge",
			label: "Label",
			hint: "Hinweis",
			class: "Class",
			required: "Benötigt",
			name: "Name",
			placeholder: "Platzhalter",
			min: "Min",
			step: "Schrittgröße",
			value: "Wert",
			data_type: "Datentyp",
			multiple: "Mehrere auswählbar",
			options: "Optionen",
		},
		option: {
			type: {
				button: "Button",
				submit: "Absenden",
				reset: "Zurücksetzen",
				password: "Passwort",
				text: "Text",
				email: "E-Mail",
				color: "Farbe",
				tel: "Tel",
			},
			data_type: {
				string: "String",
				number: "Zahl",
			},
		},
	},
	builder: {
		item: {
			label: {
				text: "Textfeld",
				textarea: "Großes Textfeld",
				markdown: "Markdown",
				richtext: "Formatiertes Textfeld",
				checkbox: "Checkbox",
				number: "Zahl",
				select: "Auswahl",
				code: "Code",
				range: "Zahlen von-bis",
				buttongroup: "Buttongroup",
				button: "Button",
			},
		},
	},
} satisfies typeof en;
