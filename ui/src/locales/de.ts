import type en from "./en";

export default {
	action: {
		create: {
			field: "Feld erstellen",
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
		},
	},
	builder: {
		item: {
			label: {
				text: "Textfeld",
				button: "Button",
				textarea: "Textarea",
			},
		},
	},
} satisfies typeof en;
