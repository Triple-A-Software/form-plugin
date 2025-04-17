import type en from "./en";

export default {
	hint: {
		preview:
			"Das ist nur eine grobe Vorschau, das eingebundene Formular auf deiner Seite wird wahrscheinlich etwas unterschiedlich aussehen.",
		cleverreachClientId: "Die Client ID für deinen Cleverreach acccount",
		cleverreachSecret:
			"Dieses Secret wird verwendet, um API-calls zu Cleverreach zu authentifizieren",
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
			cleverreachClientId: "Cleverreach client ID",
			cleverreachSecret: "Cleverreach secret",
			cleverreachGroupId: "Cleverreach Gruppen-ID",
			activateCleverreach: "Cleverreach Integration aktivieren",
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
				"Verwalte Formulare, die Besucher deiner Webseite ausfüllen können um dir Informationen zukommen zu lassen oder einen Newsletter zu abonnieren",
		},
		submissions: {
			title: "Einsendungen",
			description:
				"Siehe Einsendungen deiner Formulare, exportiere diese um sie weiter zu analysieren oder sende E-mails an die Absender",
		},
		emails: {
			title: "E-mails",
			description:
				"Verwalte E-mails, die an den Absender eines Formulars oder eine feste E-mailadresse geschickt werden können. So kannst du Besucher benachrichtigen, wenn diese ein Formular auf deiner Webseite ausfüllen",
		},
		settings: {
			title: "Einstellungen",
			cleverreach: "Cleverreach",
		},
	},
	table_columns: {
		name: "Name",
		subject: "Betreff",
		form_id: "Formular ID",
		timestamp: "Zeitpunkt",
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
