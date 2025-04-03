import type en from "./en";

export default {
	action: {
		create: {
			field: "Feld erstellen",
			option: "Option erstellen",
			form: "Formular erstellen",
		},
		save: "Speichern",
	},
	modal: {
		edit: {
			field: "Feld bearbeiten: {type}",
			form: "Formular bearbeiten",
		},
		create: {
			form: "Formular erstellen",
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
			language: "Sprache",
			sendConfirmationEmail: "Bestätigungs-Email senden",
			confirmationEmail: "Bestätigungs-Email",
			confirmationEmailRecipientFieldName: "Empfänger feld name",
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
		placeholder: {
			confirmationEmail: "Bestätigungs-Email auswählen",
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
	pages: {
		home: "Home",
		forms: "Formulare",
		submissions: "Einsendungen",
	},
	table_columns: {
		name: "Name",
	},
} satisfies typeof en;
