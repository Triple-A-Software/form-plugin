import type en from "./en";

export default {
	hint: {
		preview:
			"Das ist nur eine grobe Vorschau, das eingebundene Formular auf deiner Seite wird wahrscheinlich etwas unterschiedlich aussehen.",
	},
	action: {
		goto: {
			forms: "Formulare verwalten",
			emails: "Erreiche deine User",
			submissions: "Siehe Einsendungen",
		},
		create: {
			field: "Feld erstellen",
			option: "Option erstellen",
			form: "Formular erstellen",
			email: "E-mail erstellen",
		},
		save: "Speichern",
		saveAndClose: "Speichern und Schließen",
		no: "Nein",
		yes: "Ja",
	},
	modal: {
		edit: {
			field: "Feld bearbeiten: {type}",
			form: "Formular bearbeiten",
			email: "E-mail bearbeiten",
		},
		create: {
			form: "Formular erstellen",
			email: "E-mail erstellen",
		},
		delete: {
			title: "{name} löschen",
			description:
				"Bist du dir sicher dass du diese/s {name} löschen möchtest? Diese Aktion kann nicht rückgängig gemacht werden",
		},
	},
	deleteName: {
		form: "Formular",
		email: "E-mail",
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
			subject: "Betreff",
			content: "Inhalt",
			color: "Farbe",
			variant: "Variante",
			rows: "Zeilen",
			columns: "Spalten",
			min_label: "Minimum label",
			max_label: "Maximum label",
			preview: "Vorschau",
			default_value: "Standardwert",
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
			variant: {
				solid: "Ausgefüllt",
				alternative: "Alternativ",
			},
			default_value: {
				nothing: "Nichts",
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
		home: {
			title: "Home",
		},
		forms: {
			title: "Formulare",
			description:
				"Manage forms visitors of your website can fill out to send information to you, or sign up to anything",
		},
		submissions: {
			title: "Einsendungen",
			description:
				"See the submissions on every one of your forms, export them to analyse them further or send emails to the submitters",
		},
		emails: {
			title: "E-mails",
			description:
				"Manage emails, which can be sent to form submitters or a fixed email address. This way you can notify visitors when they fill out a form",
		},
	},
	table_columns: {
		name: "Name",
		subject: "Betreff",
		form_id: "Formular ID",
	},
	color: {
		red: "Rot",
		orange: "Orange",
		amber: "Bernstein",
		yellow: "Gelb",
		lime: "Limette",
		green: "Grün",
		emerald: "Smaragd",
		teal: "Aquamarin",
		cyan: "Cyan",
		sky: "Himmelblau",
		blue: "Blau",
		indigo: "Indigo",
		violet: "Violett",
		purple: "Lila",
		fuchsia: "Fuchsia",
		pink: "Rosa",
		rose: "Rosenfarben",
		slate: "Schiefer",
		gray: "Grau",
		zinc: "Zinc",
		neutral: "Neutral",
		stone: "Stein",
	},
} satisfies typeof en;
